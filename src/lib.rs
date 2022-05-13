use core;

use crate::parser::ast::AST;
use crate::parser::converter::{convert_ast_to_string, TOMLStringMap};
use crate::parser::parser::Parser;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod parser;

#[repr(C)]
pub struct ParserValue {
    value_type: *mut c_char,
    value: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_free_value(val: ParserValue) {
    CString::from_raw(val.value_type);
    CString::from_raw(val.value);
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_get_value(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> ParserValue {
    // Get Tag
    let raw_tag = CStr::from_ptr(tag);
    let raw_tag = if let Ok(tag) = raw_tag.to_str() {
        tag
    } else {
        panic!("[ERROR] Failed to get name.")
    };

    // Get Name
    let raw_name = CStr::from_ptr(name);
    let raw_name = if let Ok(name) = raw_name.to_str() {
        name
    } else {
        panic!("[ERROR] Failed to get name.")
    };

    // Create and return Value
    if let Some(tag) = (*map).tags.get(raw_tag) {
        let mut val: String = tag
            .values
            .get(raw_name)
            .expect("[ERROR] Name doesn't exists in the tag.")
            .clone();

        // Check type
        let mut type_str: String;
        let type_pos = val.find('#');

        if let Some(pos) = type_pos {
            type_str = val[0..pos].to_owned();
            val = val[pos + 1..val.len()].to_owned();
        } else {
            panic!("[ERROR] Failed to identify type for the value.");
        }

        // Push null character
        type_str.push('\0');
        val.push('\0');

        // Create C String
        let type_cstr: CString = CString::from_vec_unchecked(type_str.into_bytes());
        let val_cstr: CString = CString::from_vec_unchecked(val.into_bytes());

        return ParserValue {
            value_type: type_cstr.into_raw(),
            value: val_cstr.into_raw(),
        };
    } else {
        panic!("[ERROR] Tag doesn't exists in the map.");
    }
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_parse(cmap: &mut *mut TOMLStringMap, filepath: *const c_char) {
    // Conver to string
    let raw = CStr::from_ptr(filepath);
    let filepath = if let Ok(path) = raw.to_str() {
        path
    } else {
        panic!("[ERROR] Failed to get filepath.");
    };

    // Create parser
    let mut parser: Parser = Parser::new(filepath);

    // Parse
    let ast: AST = parser.parse();

    // Convert to string
    let map: TOMLStringMap = convert_ast_to_string(&ast, true);

    // Convert map to C map
    *cmap = Box::into_raw(Box::new(TOMLStringMap { tags: map.tags }));
}

#[no_mangle]
pub unsafe extern "C" fn toml_parser_free(map: *mut TOMLStringMap) {
    Box::from_raw(map);
}
