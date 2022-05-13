use core;

use crate::parser::ast::AST;
use crate::parser::converter::{convert_ast_to_string, TOMLStringMap};
use crate::parser::parser::Parser;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod parser;

#[no_mangle]
pub unsafe extern "C" fn parser_free_value(val: *mut c_char) {
    CString::from_raw(val);
}

#[no_mangle]
pub unsafe extern "C" fn parser_toml_get_value(
    map: *const TOMLStringMap,
    tag: *const c_char,
    name: *const c_char,
) -> *mut c_char {
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

    if let Some(tag) = (*map).tags.get(raw_tag) {
        let mut val: String = tag
            .values
            .get(raw_name)
            .expect("[ERROR] Name doesn't exists in the tag.").clone();

        val.push('\0');

        let cstr = CString::from_vec_unchecked(val.into_bytes());

        return cstr.into_raw();
    } else {
        panic!("[ERROR] Tag doesn't exists in the map.");
    }
}

#[no_mangle]
pub unsafe extern "C" fn parser_parse_toml (
    cmap: &mut *mut TOMLStringMap,
    filepath: *const c_char,
    typecheck: bool,
) {
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
    let map: TOMLStringMap = convert_ast_to_string(&ast, typecheck);

    // Convert map to C map
    *cmap = Box::into_raw(Box::new(TOMLStringMap { tags: map.tags }));
}

#[no_mangle]
pub unsafe extern "C" fn parser_free_toml(map: *mut TOMLStringMap) {
    Box::from_raw(map);
}