//! Objective-C Code Generator
//!
//! Parses class_dump output and generates Rust bindings automatically

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ObjCClass {
    pub name: String,
    pub superclass: Option<String>,
    pub methods: Vec<ObjCMethod>,
    pub properties: Vec<ObjCProperty>,
}

#[derive(Debug, Clone)]
pub struct ObjCMethod {
    pub name: String,
    pub type_encoding: String,
}

#[derive(Debug, Clone)]
pub struct ObjCProperty {
    pub name: String,
    pub attributes: String,
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
                properties: Vec::new(),
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
        // Parse property
        else if in_properties && line.starts_with("@property ") {
            if let Some(ref mut class) = current_class {
                // Format: "    @property propertyName [attributes]"
                let parts: Vec<&str> = line.splitn(2, " [").collect();
                if parts.len() == 2 {
                    let prop_name = parts[0]
                        .strip_prefix("@property ")
                        .unwrap_or("")
                        .trim();
                    let attributes = parts[1].strip_suffix(']').unwrap_or("").trim();

                    class.properties.push(ObjCProperty {
                        name: prop_name.to_string(),
                        attributes: attributes.to_string(),
                    });
                }
            }
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

    output.push_str("// Auto-generated Objective-C bindings from runtime introspection\n");
    output.push_str("// DO NOT EDIT - regenerate with class_dump\n\n");
    output.push_str("use crate::objc::{id, Class, SEL, objc_getClass, sel_registerName};\n");
    output.push_str("use std::ffi::CString;\n\n");

    // Add essential Foundation C functions
    output.push_str("// Essential Foundation C functions\n");
    output.push_str("#[cfg_attr(target_vendor = \"apple\", link(name = \"Foundation\", kind = \"framework\"))]\n");
    output.push_str("unsafe extern \"C\" {\n");
    output.push_str("    pub fn NSLog(format: id, ...);\n");
    output.push_str("    pub fn NSClassFromString(aClassName: id) -> Class;\n");
    output.push_str("    pub fn NSSelectorFromString(aSelectorName: id) -> SEL;\n");
    output.push_str("    pub fn NSStringFromClass(aClass: Class) -> id;\n");
    output.push_str("    pub fn NSStringFromSelector(aSelector: SEL) -> id;\n");
    output.push_str("}\n\n");

    // Add basic types
    output.push_str("// Basic Foundation types\n");
    output.push_str("pub type NSInteger = isize;\n");
    output.push_str("pub type NSUInteger = usize;\n");
    output.push_str("pub type CGFloat = f64;\n\n");

    output.push_str("#[repr(C)]\n");
    output.push_str("#[derive(Debug, Copy, Clone, PartialEq)]\n");
    output.push_str("pub struct NSRange {\n");
    output.push_str("    pub location: NSUInteger,\n");
    output.push_str("    pub length: NSUInteger,\n");
    output.push_str("}\n\n");

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

fn generate_class_bindings(output: &mut String, class: &ObjCClass) {
    let rust_name = sanitize_class_name(&class.name);

    // Skip if the name is still invalid
    if rust_name.is_empty() || rust_name.starts_with(char::is_numeric) {
        return;
    }

    // Generate class struct
    output.push_str(&format!("\n/// Objective-C class: {}\n", class.name));
    if let Some(ref superclass) = class.superclass {
        output.push_str(&format!("/// Superclass: {}\n", superclass));
    }
    output.push_str("#[repr(transparent)]\n");
    output.push_str(&format!("pub struct {}(pub id);\n\n", rust_name));

    // Generate implementation
    output.push_str(&format!("impl {} {{\n", rust_name));

    // Class method to get the ObjC class
    output.push_str("    /// Get the Objective-C Class object\n");
    output.push_str("    pub unsafe fn class() -> Class {\n");
    output.push_str(&format!(
        "        let name = CString::new(\"{}\").unwrap();\n",
        class.name
    ));
    output.push_str("        objc_getClass(name.as_ptr() as *const i8)\n");
    output.push_str("    }\n\n");

    // Generate a few common methods as examples
    // In a full implementation, you'd parse type encodings and generate all methods
    if class.name == "NSString" {
        generate_nsstring_methods(output);
    }

    output.push_str("}\n");
}

fn generate_nsstring_methods(output: &mut String) {
    output.push_str("    /// Create an NSString from a Rust str\n");
    output.push_str("    pub unsafe fn from_str(s: &str) -> Option<Self> {\n");
    output.push_str("        let class = Self::class();\n");
    output.push_str("        let sel = sel_registerName(b\"stringWithUTF8String:\\0\".as_ptr() as *const i8);\n");
    output.push_str("        let c_str = CString::new(s).ok()?;\n");
    output.push_str("        \n");
    output.push_str("        type MsgSend = unsafe extern \"C\" fn(Class, SEL, *const i8) -> id;\n");
    output.push_str("        let msg_send: MsgSend = std::mem::transmute(crate::objc::objc_msgSend as *const ());\n");
    output.push_str("        \n");
    output.push_str("        let result = msg_send(class, sel, c_str.as_ptr());\n");
    output.push_str("        if result.is_null() { None } else { Some(Self(result)) }\n");
    output.push_str("    }\n\n");

    output.push_str("    /// Get UTF-8 C string\n");
    output.push_str("    pub unsafe fn utf8_string(&self) -> Option<*const i8> {\n");
    output.push_str("        let sel = sel_registerName(b\"UTF8String\\0\".as_ptr() as *const i8);\n");
    output.push_str("        type MsgSend = unsafe extern \"C\" fn(id, SEL) -> *const i8;\n");
    output.push_str("        let msg_send: MsgSend = std::mem::transmute(crate::objc::objc_msgSend as *const ());\n");
    output.push_str("        let result = msg_send(self.0, sel);\n");
    output.push_str("        if result.is_null() { None } else { Some(result) }\n");
    output.push_str("    }\n");
}

/// Main entry point - parses dump file and generates Rust code
pub fn generate_from_dump_file(dump_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let dump_content = fs::read_to_string(dump_path)?;
    let classes = parse_class_dump(&dump_content);

    println!("cargo:warning=Parsed {} Objective-C classes from dump", classes.len());

    let rust_code = generate_rust_bindings(&classes);
    fs::write(output_path, rust_code)?;

    Ok(())
}
