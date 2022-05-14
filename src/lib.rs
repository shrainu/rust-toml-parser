#![feature(vec_into_raw_parts)]

use core;

use crate::parser::ast::AST;
use crate::parser::converter::{convert_ast_to_string, TOMLStringMap};
use crate::parser::parser::Parser;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod c;
mod parser;

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
