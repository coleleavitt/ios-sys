//! Signature database for C functions
//! Contains known function signatures for common iOS framework functions

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: String,
    pub params: Vec<(String, String)>, // (param_name, param_type)
    pub is_variadic: bool,
}

/// Build the signature database for Foundation framework
pub fn foundation_signatures() -> HashMap<String, FunctionSignature> {
    let mut db = HashMap::new();

    // NSLog and related logging functions
    db.insert(
        "NSLog".to_string(),
        FunctionSignature {
            name: "NSLog".to_string(),
            return_type: "()".to_string(),
            params: vec![("format".to_string(), "id".to_string())],
            is_variadic: true,
        },
    );

    db.insert(
        "NSLogv".to_string(),
        FunctionSignature {
            name: "NSLogv".to_string(),
            return_type: "()".to_string(),
            params: vec![
                ("format".to_string(), "id".to_string()),
                ("args".to_string(), "*mut c_void".to_string()),
            ],
            is_variadic: false,
        },
    );

    // String conversion functions
    db.insert(
        "NSClassFromString".to_string(),
        FunctionSignature {
            name: "NSClassFromString".to_string(),
            return_type: "Class".to_string(),
            params: vec![("aClassName".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSSelectorFromString".to_string(),
        FunctionSignature {
            name: "NSSelectorFromString".to_string(),
            return_type: "SEL".to_string(),
            params: vec![("aSelectorName".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromClass".to_string(),
        FunctionSignature {
            name: "NSStringFromClass".to_string(),
            return_type: "id".to_string(),
            params: vec![("aClass".to_string(), "Class".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromSelector".to_string(),
        FunctionSignature {
            name: "NSStringFromSelector".to_string(),
            return_type: "id".to_string(),
            params: vec![("aSelector".to_string(), "SEL".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromProtocol".to_string(),
        FunctionSignature {
            name: "NSStringFromProtocol".to_string(),
            return_type: "id".to_string(),
            params: vec![("proto".to_string(), "*const c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromBOOL".to_string(),
        FunctionSignature {
            name: "NSStringFromBOOL".to_string(),
            return_type: "id".to_string(),
            params: vec![("value".to_string(), "bool".to_string())],
            is_variadic: false,
        },
    );

    // NSRange functions
    db.insert(
        "NSMakeRange".to_string(),
        FunctionSignature {
            name: "NSMakeRange".to_string(),
            return_type: "NSRange".to_string(),
            params: vec![
                ("loc".to_string(), "NSUInteger".to_string()),
                ("len".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSMaxRange".to_string(),
        FunctionSignature {
            name: "NSMaxRange".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![("range".to_string(), "NSRange".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSLocationInRange".to_string(),
        FunctionSignature {
            name: "NSLocationInRange".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("loc".to_string(), "NSUInteger".to_string()),
                ("range".to_string(), "NSRange".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSEqualRanges".to_string(),
        FunctionSignature {
            name: "NSEqualRanges".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("range1".to_string(), "NSRange".to_string()),
                ("range2".to_string(), "NSRange".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSUnionRange".to_string(),
        FunctionSignature {
            name: "NSUnionRange".to_string(),
            return_type: "NSRange".to_string(),
            params: vec![
                ("range1".to_string(), "NSRange".to_string()),
                ("range2".to_string(), "NSRange".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSIntersectionRange".to_string(),
        FunctionSignature {
            name: "NSIntersectionRange".to_string(),
            return_type: "NSRange".to_string(),
            params: vec![
                ("range1".to_string(), "NSRange".to_string()),
                ("range2".to_string(), "NSRange".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromRange".to_string(),
        FunctionSignature {
            name: "NSStringFromRange".to_string(),
            return_type: "id".to_string(),
            params: vec![("range".to_string(), "NSRange".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSRangeFromString".to_string(),
        FunctionSignature {
            name: "NSRangeFromString".to_string(),
            return_type: "NSRange".to_string(),
            params: vec![("aString".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    // Memory zone functions
    db.insert(
        "NSDefaultMallocZone".to_string(),
        FunctionSignature {
            name: "NSDefaultMallocZone".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCreateZone".to_string(),
        FunctionSignature {
            name: "NSCreateZone".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![
                ("startSize".to_string(), "NSUInteger".to_string()),
                ("granularity".to_string(), "NSUInteger".to_string()),
                ("canFree".to_string(), "bool".to_string()),
            ],
            is_variadic: false,
        },
    );

    // Autorelease pool functions
    db.insert(
        "NSPushAutoreleasePool".to_string(),
        FunctionSignature {
            name: "NSPushAutoreleasePool".to_string(),
            return_type: "()".to_string(),
            params: vec![("pool".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSPopAutoreleasePool".to_string(),
        FunctionSignature {
            name: "NSPopAutoreleasePool".to_string(),
            return_type: "()".to_string(),
            params: vec![("pool".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    // Page size
    db.insert(
        "NSPageSize".to_string(),
        FunctionSignature {
            name: "NSPageSize".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSLogPageSize".to_string(),
        FunctionSignature {
            name: "NSLogPageSize".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSRoundUpToMultipleOfPageSize".to_string(),
        FunctionSignature {
            name: "NSRoundUpToMultipleOfPageSize".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![("bytes".to_string(), "NSUInteger".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSRoundDownToMultipleOfPageSize".to_string(),
        FunctionSignature {
            name: "NSRoundDownToMultipleOfPageSize".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![("bytes".to_string(), "NSUInteger".to_string())],
            is_variadic: false,
        },
    );

    // Get/set functions
    db.insert(
        "NSGetSizeAndAlignment".to_string(),
        FunctionSignature {
            name: "NSGetSizeAndAlignment".to_string(),
            return_type: "*const i8".to_string(),
            params: vec![
                ("typePtr".to_string(), "*const i8".to_string()),
                ("sizep".to_string(), "*mut NSUInteger".to_string()),
                ("alignp".to_string(), "*mut NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    // User name functions
    db.insert(
        "NSUserName".to_string(),
        FunctionSignature {
            name: "NSUserName".to_string(),
            return_type: "id".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSFullUserName".to_string(),
        FunctionSignature {
            name: "NSFullUserName".to_string(),
            return_type: "id".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSHomeDirectory".to_string(),
        FunctionSignature {
            name: "NSHomeDirectory".to_string(),
            return_type: "id".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSHomeDirectoryForUser".to_string(),
        FunctionSignature {
            name: "NSHomeDirectoryForUser".to_string(),
            return_type: "id".to_string(),
            params: vec![("userName".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSTemporaryDirectory".to_string(),
        FunctionSignature {
            name: "NSTemporaryDirectory".to_string(),
            return_type: "id".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    db.insert(
        "NSSearchPathForDirectoriesInDomains".to_string(),
        FunctionSignature {
            name: "NSSearchPathForDirectoriesInDomains".to_string(),
            return_type: "id".to_string(),
            params: vec![
                ("directory".to_string(), "NSUInteger".to_string()),
                ("domainMask".to_string(), "NSUInteger".to_string()),
                ("expandTilde".to_string(), "bool".to_string()),
            ],
            is_variadic: false,
        },
    );

    // NSDecimal functions
    db.insert(
        "NSDecimalAdd".to_string(),
        FunctionSignature {
            name: "NSDecimalAdd".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![
                ("result".to_string(), "*mut NSDecimal".to_string()),
                ("leftOperand".to_string(), "*const NSDecimal".to_string()),
                ("rightOperand".to_string(), "*const NSDecimal".to_string()),
                ("roundingMode".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecimalSubtract".to_string(),
        FunctionSignature {
            name: "NSDecimalSubtract".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![
                ("result".to_string(), "*mut NSDecimal".to_string()),
                ("leftOperand".to_string(), "*const NSDecimal".to_string()),
                ("rightOperand".to_string(), "*const NSDecimal".to_string()),
                ("roundingMode".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecimalMultiply".to_string(),
        FunctionSignature {
            name: "NSDecimalMultiply".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![
                ("result".to_string(), "*mut NSDecimal".to_string()),
                ("leftOperand".to_string(), "*const NSDecimal".to_string()),
                ("rightOperand".to_string(), "*const NSDecimal".to_string()),
                ("roundingMode".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecimalDivide".to_string(),
        FunctionSignature {
            name: "NSDecimalDivide".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![
                ("result".to_string(), "*mut NSDecimal".to_string()),
                ("leftOperand".to_string(), "*const NSDecimal".to_string()),
                ("rightOperand".to_string(), "*const NSDecimal".to_string()),
                ("roundingMode".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecimalCompare".to_string(),
        FunctionSignature {
            name: "NSDecimalCompare".to_string(),
            return_type: "NSInteger".to_string(),
            params: vec![
                ("leftOperand".to_string(), "*const NSDecimal".to_string()),
                ("rightOperand".to_string(), "*const NSDecimal".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecimalRound".to_string(),
        FunctionSignature {
            name: "NSDecimalRound".to_string(),
            return_type: "()".to_string(),
            params: vec![
                ("result".to_string(), "*mut NSDecimal".to_string()),
                ("number".to_string(), "*const NSDecimal".to_string()),
                ("scale".to_string(), "NSInteger".to_string()),
                ("roundingMode".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecimalString".to_string(),
        FunctionSignature {
            name: "NSDecimalString".to_string(),
            return_type: "id".to_string(),
            params: vec![
                ("dcm".to_string(), "*const NSDecimal".to_string()),
                ("locale".to_string(), "id".to_string()),
            ],
            is_variadic: false,
        },
    );

    // Rect functions
    db.insert(
        "NSContainsRect".to_string(),
        FunctionSignature {
            name: "NSContainsRect".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("aRect".to_string(), "NSRect".to_string()),
                ("bRect".to_string(), "NSRect".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSIntersectsRect".to_string(),
        FunctionSignature {
            name: "NSIntersectsRect".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("aRect".to_string(), "NSRect".to_string()),
                ("bRect".to_string(), "NSRect".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSUnionRect".to_string(),
        FunctionSignature {
            name: "NSUnionRect".to_string(),
            return_type: "NSRect".to_string(),
            params: vec![
                ("aRect".to_string(), "NSRect".to_string()),
                ("bRect".to_string(), "NSRect".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSIntersectionRect".to_string(),
        FunctionSignature {
            name: "NSIntersectionRect".to_string(),
            return_type: "NSRect".to_string(),
            params: vec![
                ("aRect".to_string(), "NSRect".to_string()),
                ("bRect".to_string(), "NSRect".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSMakeRect".to_string(),
        FunctionSignature {
            name: "NSMakeRect".to_string(),
            return_type: "NSRect".to_string(),
            params: vec![
                ("x".to_string(), "CGFloat".to_string()),
                ("y".to_string(), "CGFloat".to_string()),
                ("w".to_string(), "CGFloat".to_string()),
                ("h".to_string(), "CGFloat".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSMakePoint".to_string(),
        FunctionSignature {
            name: "NSMakePoint".to_string(),
            return_type: "NSPoint".to_string(),
            params: vec![
                ("x".to_string(), "CGFloat".to_string()),
                ("y".to_string(), "CGFloat".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSMakeSize".to_string(),
        FunctionSignature {
            name: "NSMakeSize".to_string(),
            return_type: "NSSize".to_string(),
            params: vec![
                ("w".to_string(), "CGFloat".to_string()),
                ("h".to_string(), "CGFloat".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSEqualRects".to_string(),
        FunctionSignature {
            name: "NSEqualRects".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("aRect".to_string(), "NSRect".to_string()),
                ("bRect".to_string(), "NSRect".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSEqualPoints".to_string(),
        FunctionSignature {
            name: "NSEqualPoints".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("aPoint".to_string(), "NSPoint".to_string()),
                ("bPoint".to_string(), "NSPoint".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSEqualSizes".to_string(),
        FunctionSignature {
            name: "NSEqualSizes".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("aSize".to_string(), "NSSize".to_string()),
                ("bSize".to_string(), "NSSize".to_string()),
            ],
            is_variadic: false,
        },
    );

    // String from X functions
    db.insert(
        "NSStringFromRect".to_string(),
        FunctionSignature {
            name: "NSStringFromRect".to_string(),
            return_type: "id".to_string(),
            params: vec![("rect".to_string(), "NSRect".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromPoint".to_string(),
        FunctionSignature {
            name: "NSStringFromPoint".to_string(),
            return_type: "id".to_string(),
            params: vec![("point".to_string(), "NSPoint".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSStringFromSize".to_string(),
        FunctionSignature {
            name: "NSStringFromSize".to_string(),
            return_type: "id".to_string(),
            params: vec![("size".to_string(), "NSSize".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSRectFromString".to_string(),
        FunctionSignature {
            name: "NSRectFromString".to_string(),
            return_type: "NSRect".to_string(),
            params: vec![("aString".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSPointFromString".to_string(),
        FunctionSignature {
            name: "NSPointFromString".to_string(),
            return_type: "NSPoint".to_string(),
            params: vec![("aString".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSSizeFromString".to_string(),
        FunctionSignature {
            name: "NSSizeFromString".to_string(),
            return_type: "NSSize".to_string(),
            params: vec![("aString".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    // NSBundle functions
    db.insert(
        "NSClassFromObject".to_string(),
        FunctionSignature {
            name: "NSClassFromObject".to_string(),
            return_type: "Class".to_string(),
            params: vec![("obj".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    // Hash table functions
    db.insert(
        "NSCreateHashTable".to_string(),
        FunctionSignature {
            name: "NSCreateHashTable".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![
                ("callBacks".to_string(), "*const c_void".to_string()),
                ("capacity".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSFreeHashTable".to_string(),
        FunctionSignature {
            name: "NSFreeHashTable".to_string(),
            return_type: "()".to_string(),
            params: vec![("table".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSResetHashTable".to_string(),
        FunctionSignature {
            name: "NSResetHashTable".to_string(),
            return_type: "()".to_string(),
            params: vec![("table".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCompareHashTables".to_string(),
        FunctionSignature {
            name: "NSCompareHashTables".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("table1".to_string(), "*const c_void".to_string()),
                ("table2".to_string(), "*const c_void".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCountHashTable".to_string(),
        FunctionSignature {
            name: "NSCountHashTable".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![("table".to_string(), "*const c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSAllHashTableObjects".to_string(),
        FunctionSignature {
            name: "NSAllHashTableObjects".to_string(),
            return_type: "id".to_string(),
            params: vec![("table".to_string(), "*const c_void".to_string())],
            is_variadic: false,
        },
    );

    // Map table functions
    db.insert(
        "NSCreateMapTable".to_string(),
        FunctionSignature {
            name: "NSCreateMapTable".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![
                ("keyCallBacks".to_string(), "*const c_void".to_string()),
                ("valueCallBacks".to_string(), "*const c_void".to_string()),
                ("capacity".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSFreeMapTable".to_string(),
        FunctionSignature {
            name: "NSFreeMapTable".to_string(),
            return_type: "()".to_string(),
            params: vec![("table".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSResetMapTable".to_string(),
        FunctionSignature {
            name: "NSResetMapTable".to_string(),
            return_type: "()".to_string(),
            params: vec![("table".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCompareMapTables".to_string(),
        FunctionSignature {
            name: "NSCompareMapTables".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("table1".to_string(), "*const c_void".to_string()),
                ("table2".to_string(), "*const c_void".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCountMapTable".to_string(),
        FunctionSignature {
            name: "NSCountMapTable".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![("table".to_string(), "*const c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSAllMapTableKeys".to_string(),
        FunctionSignature {
            name: "NSAllMapTableKeys".to_string(),
            return_type: "id".to_string(),
            params: vec![("table".to_string(), "*const c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSAllMapTableValues".to_string(),
        FunctionSignature {
            name: "NSAllMapTableValues".to_string(),
            return_type: "id".to_string(),
            params: vec![("table".to_string(), "*const c_void".to_string())],
            is_variadic: false,
        },
    );

    // Memory functions
    db.insert(
        "NSAllocateMemoryPages".to_string(),
        FunctionSignature {
            name: "NSAllocateMemoryPages".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![("bytes".to_string(), "NSUInteger".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDeallocateMemoryPages".to_string(),
        FunctionSignature {
            name: "NSDeallocateMemoryPages".to_string(),
            return_type: "()".to_string(),
            params: vec![
                ("ptr".to_string(), "*mut c_void".to_string()),
                ("bytes".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCopyMemoryPages".to_string(),
        FunctionSignature {
            name: "NSCopyMemoryPages".to_string(),
            return_type: "()".to_string(),
            params: vec![
                ("source".to_string(), "*const c_void".to_string()),
                ("dest".to_string(), "*mut c_void".to_string()),
                ("bytes".to_string(), "NSUInteger".to_string()),
            ],
            is_variadic: false,
        },
    );

    // Extra reference count functions
    db.insert(
        "NSExtraRefCount".to_string(),
        FunctionSignature {
            name: "NSExtraRefCount".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![("object".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSIncrementExtraRefCount".to_string(),
        FunctionSignature {
            name: "NSIncrementExtraRefCount".to_string(),
            return_type: "()".to_string(),
            params: vec![("object".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSDecrementExtraRefCountWasZero".to_string(),
        FunctionSignature {
            name: "NSDecrementExtraRefCountWasZero".to_string(),
            return_type: "bool".to_string(),
            params: vec![("object".to_string(), "id".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSShouldRetainWithZone".to_string(),
        FunctionSignature {
            name: "NSShouldRetainWithZone".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("object".to_string(), "id".to_string()),
                ("zone".to_string(), "*mut c_void".to_string()),
            ],
            is_variadic: false,
        },
    );

    // Run loop functions
    db.insert(
        "NSRunLoopModeMinimumRunDate".to_string(),
        FunctionSignature {
            name: "NSRunLoopModeMinimumRunDate".to_string(),
            return_type: "id".to_string(),
            params: vec![
                ("runLoop".to_string(), "id".to_string()),
                ("mode".to_string(), "id".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSRunLoopModeIsWaiting".to_string(),
        FunctionSignature {
            name: "NSRunLoopModeIsWaiting".to_string(),
            return_type: "bool".to_string(),
            params: vec![
                ("runLoop".to_string(), "id".to_string()),
                ("mode".to_string(), "id".to_string()),
            ],
            is_variadic: false,
        },
    );

    // Frame address
    db.insert(
        "NSReturnAddress".to_string(),
        FunctionSignature {
            name: "NSReturnAddress".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![("frame".to_string(), "NSUInteger".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSFrameAddress".to_string(),
        FunctionSignature {
            name: "NSFrameAddress".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![("frame".to_string(), "NSUInteger".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSCountFrames".to_string(),
        FunctionSignature {
            name: "NSCountFrames".to_string(),
            return_type: "NSUInteger".to_string(),
            params: vec![],
            is_variadic: false,
        },
    );

    // Zone functions
    db.insert(
        "NSZoneName".to_string(),
        FunctionSignature {
            name: "NSZoneName".to_string(),
            return_type: "id".to_string(),
            params: vec![("zone".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSZoneFromPointer".to_string(),
        FunctionSignature {
            name: "NSZoneFromPointer".to_string(),
            return_type: "*mut c_void".to_string(),
            params: vec![("ptr".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db.insert(
        "NSSetZoneName".to_string(),
        FunctionSignature {
            name: "NSSetZoneName".to_string(),
            return_type: "()".to_string(),
            params: vec![
                ("zone".to_string(), "*mut c_void".to_string()),
                ("name".to_string(), "id".to_string()),
            ],
            is_variadic: false,
        },
    );

    db.insert(
        "NSRecycleZone".to_string(),
        FunctionSignature {
            name: "NSRecycleZone".to_string(),
            return_type: "()".to_string(),
            params: vec![("zone".to_string(), "*mut c_void".to_string())],
            is_variadic: false,
        },
    );

    db
}

/// Generate Rust FFI binding from a function signature
pub fn generate_function_binding(sig: &FunctionSignature) -> String {
    let mut output = String::new();

    // Generate doc comment
    output.push_str(&format!("    /// {}\n", sig.name));

    // Generate function signature
    output.push_str("    pub fn ");
    output.push_str(&sig.name);
    output.push('(');

    // Parameters
    for (i, (param_name, param_type)) in sig.params.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(param_name);
        output.push_str(": ");
        output.push_str(param_type);
    }

    // Variadic marker
    if sig.is_variadic {
        if !sig.params.is_empty() {
            output.push_str(", ");
        }
        output.push_str("...");
    }

    output.push(')');

    // Return type
    if sig.return_type != "()" {
        output.push_str(" -> ");
        output.push_str(&sig.return_type);
    }

    output.push_str(";\n");

    output
}
