//! Objective-C Code Generator
//!
//! Parses class_dump output and generates Rust bindings automatically

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use super::type_encoding::{ObjCType, parse_method_encoding};

#[derive(Debug, Clone)]
pub struct ObjCClass {
    pub name: String,
    pub superclass: Option<String>,
    pub methods: Vec<ObjCMethod>,
}

#[derive(Debug, Clone)]
pub struct ObjCMethod {
    pub name: String,
    pub type_encoding: String,
}

/// Parse class_dump output into structured data
pub fn parse_class_dump(dump_content: &str) -> Vec<ObjCClass> {
    let mut classes = Vec::new();
    let mut current_class: Option<ObjCClass> = None;
    let mut in_methods = false;
    let mut in_properties = false;

    for line in dump_content.lines() {
        let line = line.trim();

        // New interface
        if line.starts_with("@interface ") {
            // Save previous class if exists
            if let Some(class) = current_class.take() {
                classes.push(class);
            }

            let name = line
                .strip_prefix("@interface ")
                .unwrap_or("")
                .trim()
                .to_string();

            current_class = Some(ObjCClass {
                name,
                superclass: None,
                methods: Vec::new(),
            });
            in_methods = false;
            in_properties = false;
        }
        // Superclass
        else if line.starts_with("Superclass: ") {
            if let Some(ref mut class) = current_class {
                class.superclass = Some(line.strip_prefix("Superclass: ").unwrap().to_string());
            }
        }
        // Methods section
        else if line.starts_with("Methods (") {
            in_methods = true;
            in_properties = false;
        }
        // Properties section
        else if line.starts_with("Properties (") {
            in_methods = false;
            in_properties = true;
        }
        // End of interface
        else if line == "@end" {
            in_methods = false;
            in_properties = false;
        }
        // Parse method
        else if in_methods && line.starts_with("- ") {
            if let Some(ref mut class) = current_class {
                // Format: "    - methodName:param: [type_encoding]"
                let parts: Vec<&str> = line.splitn(2, " [").collect();
                if parts.len() == 2 {
                    let method_name = parts[0].strip_prefix("- ").unwrap_or("").trim();
                    let type_encoding = parts[1].strip_suffix(']').unwrap_or("").trim();

                    class.methods.push(ObjCMethod {
                        name: method_name.to_string(),
                        type_encoding: type_encoding.to_string(),
                    });
                }
            }
        }
        // Parse property (currently ignored)
        else if in_properties && line.starts_with("@property ") {
            // Properties are not currently used
        }
    }

    // Save last class
    if let Some(class) = current_class {
        classes.push(class);
    }

    classes
}

/// Generate Rust code from ObjC classes
pub fn generate_rust_bindings(classes: &[ObjCClass]) -> String {
    let mut output = String::new();

    writeln!(output, "// Auto-generated Objective-C bindings from runtime introspection").unwrap();
    writeln!(output, "// DO NOT EDIT - regenerate with class_dump\n").unwrap();
    writeln!(output, "use crate::objc::{{").unwrap();
    writeln!(output, "    id, Class, SEL, objc_getClass, sel_registerName,").unwrap();
    writeln!(output, "    objc_ivar, objc_method, objc_method_description, objc_property_t,").unwrap();
    writeln!(output, "}};").unwrap();
    writeln!(output, "use std::ffi::CString;").unwrap();
    writeln!(output, "use core::ffi::c_void;\n").unwrap();

    // Add essential Foundation C functions
    writeln!(output, "// Essential Foundation C functions").unwrap();
    writeln!(output, "#[cfg_attr(all(target_vendor = \"apple\", feature = \"runtime\"),").unwrap();
    writeln!(output, "           link(name = \"Foundation\", kind = \"framework\"))]").unwrap();
    writeln!(output, "unsafe extern \"C\" {{").unwrap();
    writeln!(output, "    pub fn NSLog(format: id, ...);").unwrap();
    writeln!(output, "    pub fn NSClassFromString(aClassName: id) -> Class;").unwrap();
    writeln!(output, "    pub fn NSSelectorFromString(aSelectorName: id) -> SEL;").unwrap();
    writeln!(output, "    pub fn NSStringFromClass(aClass: Class) -> id;").unwrap();
    writeln!(output, "    pub fn NSStringFromSelector(aSelector: SEL) -> id;").unwrap();
    writeln!(output, "}}\n").unwrap();

    // Add basic types
    writeln!(output, "// Basic Foundation types").unwrap();
    writeln!(output, "pub type NSInteger = isize;").unwrap();
    writeln!(output, "pub type NSUInteger = usize;").unwrap();
    writeln!(output, "pub type CGFloat = f64;").unwrap();
    writeln!(output, "pub type NSTimeInterval = f64;\n").unwrap();

    writeln!(output, "#[repr(C)]").unwrap();
    writeln!(output, "#[derive(Debug, Copy, Clone, PartialEq)]").unwrap();
    writeln!(output, "pub struct NSRange {{").unwrap();
    writeln!(output, "    pub location: NSUInteger,").unwrap();
    writeln!(output, "    pub length: NSUInteger,").unwrap();
    writeln!(output, "}}\n").unwrap();

    // Add common opaque types
    writeln!(output, "// Common opaque Foundation types").unwrap();
    writeln!(output, "#[repr(C)]").unwrap();
    writeln!(output, "pub struct NSZone {{").unwrap();
    writeln!(output, "    _private: [u8; 0],").unwrap();
    writeln!(output, "}}\n").unwrap();

    writeln!(output, "#[repr(C)]").unwrap();
    writeln!(output, "#[derive(Debug, Copy, Clone)]").unwrap();
    writeln!(output, "pub struct NSProgressFraction {{").unwrap();
    writeln!(output, "    pub completed: i64,").unwrap();
    writeln!(output, "    pub total: i64,").unwrap();
    writeln!(output, "}}\n").unwrap();

    writeln!(output, "#[repr(C)]").unwrap();
    writeln!(output, "#[derive(Debug, Copy, Clone)]").unwrap();
    writeln!(output, "pub struct NSDecimal {{").unwrap();
    writeln!(output, "    _private: [u8; 20],").unwrap();
    writeln!(output, "}}\n").unwrap();

    writeln!(output, "#[repr(C)]").unwrap();
    writeln!(output, "#[derive(Debug, Copy, Clone)]").unwrap();
    writeln!(output, "pub struct NSFastEnumerationState {{").unwrap();
    writeln!(output, "    pub state: u64,").unwrap();
    writeln!(output, "    pub itemsPtr: *mut id,").unwrap();
    writeln!(output, "    pub mutationsPtr: *mut u64,").unwrap();
    writeln!(output, "    pub extra: [u64; 5],").unwrap();
    writeln!(output, "}}\n").unwrap();

    for class in classes {
        generate_class_bindings(&mut output, class);
    }

    output
}

/// Sanitize an Objective-C class name to be a valid Rust identifier
fn sanitize_class_name(name: &str) -> String {
    // Replace invalid characters with underscores
    name.replace(".", "_")
        .replace("-", "_")
        .replace(" ", "_")
        .replace("+", "Plus")
        .replace("$", "Dollar")
        .replace("@", "At")
}

/// Sanitize an Objective-C selector to be a valid Rust method name
fn sanitize_selector(selector: &str) -> String {
    // Remove colons and convert to snake_case style
    let mut result = selector
        .replace(":", "_")
        .replace("-", "_")
        .replace("+", "plus_")
        .replace("$", "dollar_")
        .replace(".", "_") // Replace dots (e.g., .cxx_destruct)
        .trim_matches('_')
        .to_string();

    // If it starts with a digit, prefix with underscore
    if result.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        result = format!("_{}", result);
    }

    // If it's a Rust keyword, append underscore
    match result.as_str() {
        "self" | "Self" | "super" | "crate" | "type" | "move" | "box" | "impl" | "fn" | "let"
        | "mut" | "ref" | "static" | "const" | "unsafe" | "async" | "await" | "dyn"
        | "abstract" | "final" | "override" | "macro" | "typeof" | "yield" | "return" | "break"
        | "continue" | "loop" | "while" | "for" | "if" | "else" | "match" | "pub" | "use"
        | "extern" | "mod" | "trait" | "struct" | "enum" | "union" | "where" | "as" | "in"
        | "become" => {
            format!("{}_", result)
        }
        _ => result,
    }
}

/// Generate argument names and msgSend selector string for a method
fn generate_selector_call(selector: &str, arg_count: usize) -> (Vec<String>, String) {
    let parts: Vec<&str> = selector.split(':').filter(|s| !s.is_empty()).collect();

    let mut arg_names = Vec::new();
    let mut msg_send_selector = String::new();

    for (i, part) in parts.iter().enumerate() {
        msg_send_selector.push_str(part);
        msg_send_selector.push(':');

        if i < arg_count {
            arg_names.push(format!("arg{}", i));
        }
    }

    // Handle selectors without colons
    if parts.is_empty() {
        msg_send_selector = selector.to_string();
    }

    (arg_names, msg_send_selector)
}

/// Generate a single method binding from a method signature with custom name
fn generate_method_binding_named(output: &mut String, method: &ObjCMethod, method_name: &str) {
    // Parse the method type encoding
    let sig = match parse_method_encoding(&method.type_encoding) {
        Some(s) => s,
        None => {
            // If we can't parse the encoding, skip this method
            writeln!(output, "    // Skipped: {} (unparseable encoding: {})",
                     method.name, method.type_encoding).unwrap();
            return;
        }
    };

    // Skip if no arguments (should at least have self and _cmd)
    if sig.arg_types.len() < 2 {
        return;
    }

    // Method arguments (skip first 2: self @ and _cmd :)
    let method_args = &sig.arg_types[2..];
    let arg_count = method_args.len();

    // Generate selector call and argument names
    let (_selector_arg_names, selector_str) = generate_selector_call(&method.name, arg_count);

    // Generate return type
    let return_type = sig.return_type.to_rust_type();

    // Build method signature
    writeln!(output).unwrap();
    writeln!(output, "    /// Objective-C method `{}`", method.name).unwrap();
    writeln!(output, "    /// Type encoding: `{}`", method.type_encoding).unwrap();
    writeln!(output, "    #[inline]").unwrap();
    write!(output, "    pub unsafe fn {}(&self", method_name).unwrap();

    // Add arguments to signature - use simple arg0, arg1, arg2 naming
    for (i, arg_type) in method_args.iter().enumerate() {
        let arg_name = format!("arg{}", i);
        let rust_type = arg_type.to_rust_type();
        write!(output, ", {}: {}", arg_name, rust_type).unwrap();
    }

    writeln!(output, ") -> {} {{", return_type).unwrap();

    // Generate the method body using objc_msgSend
    writeln!(output, "        let sel = sel_registerName(b\"{}\\0\".as_ptr() as *const i8);",
             selector_str).unwrap();

    // Generate the appropriate msgSend call based on return type
    let msg_send_fn = match &sig.return_type {
        ObjCType::Void => "objc_msgSend",
        ObjCType::Float | ObjCType::Double => "objc_msgSend_fpret",
        ObjCType::LongLong | ObjCType::UnsignedLongLong if cfg!(target_arch = "x86_64") => {
            "objc_msgSend"
        }
        ObjCType::Struct(_) => "objc_msgSend_stret",
        _ => "objc_msgSend",
    };

    // Build the function type signature for msgSend
    write!(output, "        type MsgSend = unsafe extern \"C\" fn(id, SEL").unwrap();
    for arg_type in method_args {
        write!(output, ", {}", arg_type.to_rust_type()).unwrap();
    }
    writeln!(output, ") -> {};", return_type).unwrap();

    // Cast and call
    writeln!(output, "        let msg_send: MsgSend = std::mem::transmute(crate::objc::{} as *const ());",
             msg_send_fn).unwrap();
    write!(output, "        msg_send(self.0, sel").unwrap();
    for i in 0..arg_count {
        write!(output, ", arg{}", i).unwrap();
    }
    writeln!(output, ")").unwrap();

    writeln!(output, "    }}").unwrap();
}

fn generate_class_bindings(output: &mut String, class: &ObjCClass) {
    let rust_name = sanitize_class_name(&class.name);

    // Skip if the name is still invalid
    if rust_name.is_empty() || rust_name.starts_with(char::is_numeric) {
        return;
    }

    // Generate class struct
    writeln!(output).unwrap();
    writeln!(output, "/// Objective-C class: {}", class.name).unwrap();
    if let Some(ref superclass) = class.superclass {
        writeln!(output, "/// Superclass: {}", superclass).unwrap();
    }
    writeln!(output, "#[repr(transparent)]").unwrap();
    writeln!(output, "pub struct {}(pub id);\n", rust_name).unwrap();

    // Generate implementation
    writeln!(output, "impl {} {{", rust_name).unwrap();

    // Class method to get the ObjC class
    writeln!(output, "    /// Get the Objective-C Class object").unwrap();
    writeln!(output, "    pub unsafe fn class() -> Class {{").unwrap();
    writeln!(output, "        let name = CString::new(\"{}\").unwrap();", class.name).unwrap();
    writeln!(output, "        objc_getClass(name.as_ptr() as *const i8)").unwrap();
    writeln!(output, "    }}").unwrap();

    // Generate ALL methods from the class dump
    // Track method names to handle duplicates
    let mut method_names: HashMap<String, usize> = HashMap::new();

    for method in &class.methods {
        let method_name = sanitize_selector(&method.name);

        // Check for duplicates and append suffix if needed
        let count = method_names.entry(method_name.clone()).or_insert(0);
        let current_count = *count;
        *count += 1;

        let unique_method_name = if current_count > 0 {
            format!("{}_{}", method_name, current_count)
        } else {
            method_name
        };

        generate_method_binding_named(output, method, &unique_method_name);
    }

    // Keep NSString convenience methods if present
    if class.name == "NSString" {
        generate_nsstring_convenience_methods(output);
    }

    writeln!(output, "}}").unwrap();
}

fn generate_nsstring_convenience_methods(output: &mut String) {
    writeln!(output, "    /// Create an NSString from a Rust str").unwrap();
    writeln!(output, "    pub unsafe fn from_str(s: &str) -> Option<Self> {{").unwrap();
    writeln!(output, "        let class = Self::class();").unwrap();
    writeln!(output, "        let sel = sel_registerName(b\"stringWithUTF8String:\\0\".as_ptr() as *const i8);").unwrap();
    writeln!(output, "        let c_str = CString::new(s).ok()?;").unwrap();
    writeln!(output, "        ").unwrap();
    writeln!(output, "        type MsgSend = unsafe extern \"C\" fn(Class, SEL, *const i8) -> id;").unwrap();
    writeln!(output, "        let msg_send: MsgSend = std::mem::transmute(crate::objc::objc_msgSend as *const ());").unwrap();
    writeln!(output, "        ").unwrap();
    writeln!(output, "        let result = msg_send(class, sel, c_str.as_ptr());").unwrap();
    writeln!(output, "        if result.is_null() {{ None }} else {{ Some(Self(result)) }}").unwrap();
    writeln!(output, "    }}\n").unwrap();

    writeln!(output, "    /// Get UTF-8 C string").unwrap();
    writeln!(output, "    pub unsafe fn utf8_string(&self) -> Option<*const i8> {{").unwrap();
    writeln!(output, "        let sel = sel_registerName(b\"UTF8String\\0\".as_ptr() as *const i8);").unwrap();
    writeln!(output, "        type MsgSend = unsafe extern \"C\" fn(id, SEL) -> *const i8;").unwrap();
    writeln!(output, "        let msg_send: MsgSend = std::mem::transmute(crate::objc::objc_msgSend as *const ());").unwrap();
    writeln!(output, "        let result = msg_send(self.0, sel);").unwrap();
    writeln!(output, "        if result.is_null() {{ None }} else {{ Some(result) }}").unwrap();
    writeln!(output, "    }}").unwrap();
}

/// Main entry point - parses dump file and generates Rust code
pub fn generate_from_dump_file(dump_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let dump_content = fs::read_to_string(dump_path)?;
    let classes = parse_class_dump(&dump_content);

    println!(
        "cargo:warning=Parsed {} Objective-C classes from dump",
        classes.len()
    );

    let rust_code = generate_rust_bindings(&classes);
    fs::write(output_path, rust_code)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_selector_basic() {
        assert_eq!(sanitize_selector("init"), "init");
        assert_eq!(sanitize_selector("initWithString:"), "initWithString");
        assert_eq!(sanitize_selector("init:with:"), "init_with");
    }

    #[test]
    fn test_sanitize_selector_special_chars() {
        assert_eq!(sanitize_selector(".cxx_destruct"), "cxx_destruct");
        assert_eq!(sanitize_selector("operator+"), "operatorplus");
        assert_eq!(sanitize_selector("test-method"), "test_method");
    }

    #[test]
    fn test_sanitize_selector_keywords() {
        assert_eq!(sanitize_selector("self"), "self_");
        assert_eq!(sanitize_selector("type"), "type_");
        assert_eq!(sanitize_selector("async"), "async_");
    }

    #[test]
    fn test_sanitize_class_name() {
        assert_eq!(sanitize_class_name("NSString"), "NSString");
        assert_eq!(sanitize_class_name("NS.Something"), "NS_Something");
        assert_eq!(sanitize_class_name("Test-Class"), "Test_Class");
    }
}
