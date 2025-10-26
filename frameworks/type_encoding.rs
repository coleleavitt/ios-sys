//! Objective-C Type Encoding Parser
//!
//! Parses Apple's @encode type encoding strings into Rust types

#[derive(Debug, Clone, PartialEq)]
pub enum ObjCType {
    Void,
    Bool,
    Char,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    Id,           // @
    Class,        // #
    SEL,          // :
    CharPointer,  // *
    Pointer(Box<ObjCType>),  // ^type
    Struct(String),          // {name=fields}
    Unknown(String),
}

impl ObjCType {
    /// Convert ObjC type encoding to Rust type string
    pub fn to_rust_type(&self) -> String {
        match self {
            ObjCType::Void => "()".to_string(),
            ObjCType::Bool => "bool".to_string(),
            ObjCType::Char => "i8".to_string(),
            ObjCType::UnsignedChar => "u8".to_string(),
            ObjCType::Short => "i16".to_string(),
            ObjCType::UnsignedShort => "u16".to_string(),
            ObjCType::Int => "i32".to_string(),
            ObjCType::UnsignedInt => "u32".to_string(),
            ObjCType::Long => "isize".to_string(),
            ObjCType::UnsignedLong => "usize".to_string(),
            ObjCType::LongLong => "i64".to_string(),
            ObjCType::UnsignedLongLong => "u64".to_string(),
            ObjCType::Float => "f32".to_string(),
            ObjCType::Double => "f64".to_string(),
            ObjCType::Id => "id".to_string(),
            ObjCType::Class => "Class".to_string(),
            ObjCType::SEL => "SEL".to_string(),
            ObjCType::CharPointer => "*const i8".to_string(),
            ObjCType::Pointer(inner) => {
                // Special handling for unknown pointer types
                match &**inner {
                    ObjCType::Unknown(_) => "*mut c_void".to_string(),
                    ObjCType::Struct(s) if s.starts_with('?') || s.is_empty() => "*mut c_void".to_string(),
                    _ => format!("*mut {}", inner.to_rust_type()),
                }
            }
            ObjCType::Struct(name) => {
                // If struct name is unknown (?), use c_void
                if name.starts_with('?') || name.is_empty() {
                    return "c_void".to_string();
                }

                // Strip ALL leading underscores from struct names (e.g., __CFString -> CFString)
                let clean_name = name.trim_start_matches('_');

                // Map common C/POSIX/system types to c_void since we don't have their definitions
                // Check prefixes/patterns first, then specific names
                if clean_name.starts_with("CF") || clean_name.starts_with("CG") ||
                   clean_name.starts_with("Sec") || clean_name.starts_with("nw_") ||
                   clean_name.starts_with("xpc_") || clean_name.starts_with("HTTP") ||
                   clean_name.starts_with("URL") || clean_name.starts_with("IO") ||
                   clean_name.starts_with("dispatch_") || clean_name.starts_with("os_") ||
                   clean_name.contains("Cookie") {
                    "c_void".to_string()
                } else {
                    match clean_name {
                        // POSIX types
                        "stat" | "timespec" | "timeval" | "addrinfo" | "sockaddr" |
                        "sockaddr_in" | "sockaddr_in6" | "passwd" | "group" | "ftsent" |
                        // SQLite
                        "sqlite3" | "sqlite3_stmt" |
                        // ASN.1 / Crypto types (from Security framework)
                        "Attribute" | "Attribute_value" | "IssuerAndSerialNumber" |
                        "AlgorithmIdentifier" | "heim_integer" | "heim_oid" |
                        // Other opaque/private Foundation types
                        "NSLTToken" | "NSRefCountedRunArray" | "NSRunArrayItem" |
                        "NSMethodFrameArgInfo" | "InternalInit" | "audit_token_t" => "c_void".to_string(),
                        _ => clean_name.to_string(),
                    }
                }
            }
            ObjCType::Unknown(s) => format!("/* {} */ c_void", s),
        }
    }
}

/// Parse a single type encoding character/sequence
pub fn parse_type_encoding(encoding: &str) -> (ObjCType, usize) {
    if encoding.is_empty() {
        return (ObjCType::Unknown("empty".to_string()), 0);
    }

    let first_char = encoding.chars().next().unwrap();

    match first_char {
        'v' => (ObjCType::Void, 1),
        'B' => (ObjCType::Bool, 1),
        'c' => (ObjCType::Char, 1),
        'C' => (ObjCType::UnsignedChar, 1),
        's' => (ObjCType::Short, 1),
        'S' => (ObjCType::UnsignedShort, 1),
        'i' => (ObjCType::Int, 1),
        'I' => (ObjCType::UnsignedInt, 1),
        'l' => (ObjCType::Long, 1),
        'L' => (ObjCType::UnsignedLong, 1),
        'q' => (ObjCType::LongLong, 1),
        'Q' => (ObjCType::UnsignedLongLong, 1),
        'f' => (ObjCType::Float, 1),
        'd' => (ObjCType::Double, 1),
        '@' => (ObjCType::Id, 1),
        '#' => (ObjCType::Class, 1),
        ':' => (ObjCType::SEL, 1),
        '*' => (ObjCType::CharPointer, 1),
        '^' => {
            // Pointer to type
            let (inner_type, consumed) = parse_type_encoding(&encoding[1..]);
            (ObjCType::Pointer(Box::new(inner_type)), consumed + 1)
        }
        '{' => {
            // Struct type: {name=fields}
            if let Some(end_pos) = encoding.find('}') {
                let struct_encoding = &encoding[1..end_pos];
                let struct_name = if let Some(eq_pos) = struct_encoding.find('=') {
                    &struct_encoding[..eq_pos]
                } else {
                    struct_encoding
                };
                (ObjCType::Struct(struct_name.to_string()), end_pos + 1)
            } else {
                (ObjCType::Unknown(encoding[..10.min(encoding.len())].to_string()), 1)
            }
        }
        _ => (ObjCType::Unknown(first_char.to_string()), 1),
    }
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub return_type: ObjCType,
    pub arg_types: Vec<ObjCType>,
}

/// Parse a full method type encoding like "@24@0:8@16"
/// Format: return_type[stack_size]arg1_type[offset]arg2_type[offset]...
pub fn parse_method_encoding(encoding: &str) -> Option<MethodSignature> {
    // Remove the brackets if present: [v16@0:8] -> v16@0:8
    let clean = encoding.trim().trim_matches(|c| c == '[' || c == ']');

    if clean.is_empty() {
        return None;
    }

    let mut types = Vec::new();
    let mut pos = 0;

    // Parse types, skipping numeric offsets/sizes
    while pos < clean.len() {
        let ch = clean.chars().nth(pos).unwrap();

        // Skip numbers (stack sizes/offsets)
        if ch.is_ascii_digit() {
            while pos < clean.len() && clean.chars().nth(pos).unwrap().is_ascii_digit() {
                pos += 1;
            }
            continue;
        }

        // Parse the type
        let (parsed_type, consumed) = parse_type_encoding(&clean[pos..]);
        types.push(parsed_type);
        pos += consumed;
    }

    if types.is_empty() {
        return None;
    }

    // First type is return type, rest are arguments
    // First two arguments are always self (@) and _cmd (:)
    let return_type = types[0].clone();
    let arg_types = types[1..].to_vec();

    Some(MethodSignature {
        return_type,
        arg_types,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_types() {
        assert_eq!(parse_type_encoding("v").0, ObjCType::Void);
        assert_eq!(parse_type_encoding("@").0, ObjCType::Id);
        assert_eq!(parse_type_encoding(":").0, ObjCType::SEL);
        assert_eq!(parse_type_encoding("#").0, ObjCType::Class);
        assert_eq!(parse_type_encoding("B").0, ObjCType::Bool);
    }

    #[test]
    fn test_parse_method_encoding() {
        // - release [v16@0:8]
        let sig = parse_method_encoding("v16@0:8").unwrap();
        assert_eq!(sig.return_type, ObjCType::Void);
        assert_eq!(sig.arg_types.len(), 2); // self and _cmd
        assert_eq!(sig.arg_types[0], ObjCType::Id);
        assert_eq!(sig.arg_types[1], ObjCType::SEL);

        // - isKindOfClass: [B24@0:8#16]
        let sig = parse_method_encoding("B24@0:8#16").unwrap();
        assert_eq!(sig.return_type, ObjCType::Bool);
        assert_eq!(sig.arg_types.len(), 3); // self, _cmd, Class parameter
        assert_eq!(sig.arg_types[2], ObjCType::Class);
    }

    #[test]
    fn test_to_rust_type() {
        assert_eq!(ObjCType::Void.to_rust_type(), "()");
        assert_eq!(ObjCType::Bool.to_rust_type(), "bool");
        assert_eq!(ObjCType::Id.to_rust_type(), "id");
        assert_eq!(ObjCType::Class.to_rust_type(), "Class");
        assert_eq!(ObjCType::SEL.to_rust_type(), "SEL");
    }

    #[test]
    fn test_parse_complex_struct() {
        // {CGRect={CGPoint=dd}{CGSize=dd}}
        let (ty, _) = parse_type_encoding("{CGRect={CGPoint=dd}{CGSize=dd}}");
        match ty {
            ObjCType::Struct(name) => assert_eq!(name, "CGRect"),
            _ => panic!("Expected Struct type"),
        }
    }

    #[test]
    fn test_struct_underscore_stripping() {
        // __CFString should strip to CFString
        let ty = ObjCType::Struct("__CFString".to_string());
        assert_eq!(ty.to_rust_type(), "c_void"); // CF* types map to c_void

        // _NSRange should strip to NSRange
        let ty = ObjCType::Struct("_NSRange".to_string());
        assert_eq!(ty.to_rust_type(), "NSRange");
    }

    #[test]
    fn test_pointer_types() {
        // ^v = void pointer
        let (ty, _) = parse_type_encoding("^v");
        assert_eq!(ty.to_rust_type(), "*mut ()");

        // ^{__CFString=} = pointer to __CFString
        let (ty, _) = parse_type_encoding("^{__CFString=}");
        assert_eq!(ty.to_rust_type(), "*mut c_void");
    }

    #[test]
    fn test_cf_types_map_to_c_void() {
        // All CF* types should map to c_void
        assert_eq!(ObjCType::Struct("CFString".to_string()).to_rust_type(), "c_void");
        assert_eq!(ObjCType::Struct("CGPoint".to_string()).to_rust_type(), "c_void");
        assert_eq!(ObjCType::Struct("CFDictionary".to_string()).to_rust_type(), "c_void");
    }

    #[test]
    fn test_security_types_map_to_c_void() {
        // Security framework types
        assert_eq!(ObjCType::Struct("SecKey".to_string()).to_rust_type(), "c_void");
        assert_eq!(ObjCType::Struct("SecTrust".to_string()).to_rust_type(), "c_void");
    }

    #[test]
    fn test_unknown_struct_becomes_identifier() {
        // Unknown custom structs pass through
        assert_eq!(ObjCType::Struct("MyCustomStruct".to_string()).to_rust_type(), "MyCustomStruct");
    }
}
