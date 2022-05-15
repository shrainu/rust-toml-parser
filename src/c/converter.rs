use crate::parser::converter::TOMLStringTag;
use crate::TOMLStringMap;
use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::thread::current;

const C_TYPE_INT: &str = "int";
const C_TYPE_STR: &str = "string";
const C_TYPE_BOOL: &str = "bool";
const C_TYPE_ARRAY_INT: &str = "array_int";
const C_TYPE_ARRAY_ARRAY_INT: &str = "array_array_int";

/*
#[no_mangle]
pub unsafe extern "C" fn toml_parser_free_value(val: ParserValue) {
    CString::from_raw(val.value_type);
    CString::from_raw(val.value);
}
*/
unsafe fn convert_string_to_int_array(array: String) -> *mut i32 {

    let mut values: Vec<i32> = vec![0];

    let mut current: String = if let Some(pos) = array.find(';') {
        array[pos + 1..array.len()].to_owned()
    } else {
        println!("[WARNING] Array is empty.");
        return values.into_raw_parts().0;
    };

    loop {
        if let Some(pos) = current.find(';') {
            let num: i32 = if let Ok(num) = current[0..pos].to_owned().parse::<i32>() {
                num
            } else {
                panic!(
                    "[ERROR] Failed to parse int from value `{}`.",
                    current[0..pos].to_owned());
            };

            values.push(num);

            current = current[pos + 1..current.len()].to_owned();
        } else {
            let num: i32 = if let Ok(num) = current.parse::<i32>() {
                num
            } else {
                panic!(
                    "[ERROR] Failed to parse int from value `{}`.",
                    current);
            };

            values.push(num);
            break;
        }
    }

    // Put size
    values[0] = values.len() as i32;

    // Return array
    return values.into_raw_parts().0;
}

unsafe fn convert_c_str_to_str(c_string: *const c_char) -> &'static str {
    let raw_str: &CStr = CStr::from_ptr(c_string);
    let str: &str = if let Ok(str) = raw_str.to_str() {
        str
    } else {
        panic!("[ERROR] Failed to conver C string to &str");
    };

    return str;
}

unsafe fn toml_parser_get_value(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> String {
    // Get tag
    let tag: &str = convert_c_str_to_str(tag);

    // Get name
    let name: &str = convert_c_str_to_str(name);

    // Tag
    let tag: &TOMLStringTag = (*map).tags.get(tag)
        .expect("[ERROR] Tag doesn't exists.");

    // Name
    let val: &String = tag.values.get(name)
        .expect("[ERROR] Name doesn't exists.");

    // Return the value
    return val.clone();
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_get_int(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> i32 {
    // Get value
    let val: String = toml_parser_get_value(map, tag, name);

    // Find type
    let type_sign = val
        .find('#')
        .expect("[ERROR] Map is not parsed with typechecking.");

    let type_str: String = val[0..type_sign].to_owned();

    // Check type
    if type_str != C_TYPE_INT {
        panic!("[ERROR] Type of value is not an int.");
    }

    // Value
    let value: i32 = if let Ok(num) = val[type_sign + 1..val.len()].to_owned().parse() {
        num
    } else {
        panic!(
            "[ERROR] Failed to parse int from value `{}`.",
            val[type_sign + 1..val.len()].to_owned()
        );
    };

    // Return the int
    return value;
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_get_string (
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> *mut c_char {
    // Get value
    let val: String = toml_parser_get_value(map, tag, name);

    // Find type
    let type_sign = val
        .find('#')
        .expect("[ERROR] Map is not parsed with typechecking.");

    let type_str: String = val[0..type_sign].to_owned();

    // Check type
    if type_str != C_TYPE_STR {
        panic!("[ERROR] Type of value is not an int.");
    }

    // Prepare value in Rust
    let mut rust_value: String = val[type_sign + 1..val.len()].to_owned();

    // Push null char
    rust_value.push('\0');

    // Value
    let value: CString = CString::from_vec_unchecked(rust_value.into_bytes());

    // Return the value
    return value.into_raw();
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_get_bool(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> bool {
    // Get value
    let val: String = toml_parser_get_value(map, tag, name);

    // Find type
    let type_sign = val
        .find('#')
        .expect("[ERROR] Map is not parsed with typechecking.");

    let type_str: String = val[0..type_sign].to_owned();

    // Check type
    if type_str != C_TYPE_BOOL {
        panic!("[ERROR] Type of value is not an int.");
    }

    // Value in Rust string
    let value_str: String = val[type_sign + 1..val.len()].to_owned();

    // Value
    let value: bool = if value_str == "true" { true } else { false };

    // Return value
    return value;
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_get_int_array(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> *mut i32 {
    // Get value
    let val: String = toml_parser_get_value(map, tag, name);

    // Find type
    let type_sign = val
        .find('#')
        .expect("[ERROR] Map is not parsed with typechecking.");

    let type_str: String = val[0..type_sign].to_owned();

    // Check type
    if type_str != C_TYPE_ARRAY_INT {
        panic!("[ERROR] Type of value is not an int.");
    }

    // Convert to int array
    let array: *mut i32 = convert_string_to_int_array(val);

    // Return value
    return array;
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_get_int_array_array(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
    size: *mut i32
) -> *mut *mut i32 {

    // Get value
    let val: String = toml_parser_get_value(map, tag, name);

    // Find type
    let type_sign = val
        .find('#')
        .expect("[ERROR] Map is not parsed with typechecking.");

    let type_str: String = val[0..type_sign].to_owned();

    // Check type
    if type_str != C_TYPE_ARRAY_ARRAY_INT {
        panic!("[ERROR] Type of value is not an int.");
    }

    // Array of arrays
    let mut array: Vec<*mut i32> = vec![];

    let mut current: String = if let Some(pos) = val.find('|') {
        val[pos + 1..val.len()].to_owned()
    } else {
        println!("[WARNING] Array is empty.");

        *size = 0;
        return null_mut();
    };

    loop {
        if let Some(pos) = current.find('|') {

            let val: *mut i32 = convert_string_to_int_array(current[0..pos].to_owned());
            array.push(val);

            if !(pos + 2 <= current.len() - 1) {
                break;
            } else {
                current = current[pos + 2..current.len()].to_owned();
            }
        } else {
            break;
        }
    }

    // Set the size
    *size = array.len() as i32;

    // Return the array
    return array.into_raw_parts().0;
}