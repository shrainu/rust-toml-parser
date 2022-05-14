use crate::parser::ast::AST;
use std::any::Any;

use std::collections::HashMap;

pub struct TOMLStringMap {
    pub tags: HashMap<String, TOMLStringTag>,
}

impl TOMLStringMap {
    pub fn new() -> Self {
        return Self {
            tags: HashMap::new(),
        };
    }

    pub fn add_tag(&mut self, tag: &str, crash_on_collision: bool) {
        let tag = TOMLStringTag::new(tag);

        if !self.tags.contains_key(tag.name.as_str()) {
            self.tags.insert(tag.name.clone(), tag);
        } else if crash_on_collision {
            panic!("[ERROR] Tag with name `{}`, already exists.", &tag.name);
        }
    }

    pub fn add_value(&mut self, tag: &str, n: &str, v: &str) {
        if let Some(tag) = self.tags.get_mut(tag) {
            tag.insert_value(n, v);
        } else {
            panic!("[ERROR] Tag with name `{}`, doesn't exists", tag);
        }
    }
}

pub struct TOMLStringTag {
    pub name: String,
    pub values: HashMap<String, String>,
}

impl TOMLStringTag {
    pub fn new(n: &str) -> Self {
        return Self {
            name: String::from(n),
            values: HashMap::new(),
        };
    }

    pub fn insert_value(&mut self, n: &str, v: &str) {
        self.values.insert(String::from(n), String::from(v));
    }
}

fn convert_ast_array_to_string(array: &Vec<AST>, typecheck: bool) -> String {
    // Return string
    let mut string: String = String::new();

    // Type Checking
    let mut first: bool = true;
    let mut current_type: String = String::new();

    let mut check_type = |str: &mut String, t: &str| {
        if !typecheck {
            return;
        }

        let real_t: String = t.to_owned() + "#";

        if first {
            first = false;

            current_type = real_t.clone();
            str.push_str(real_t.as_str());
        }

        if real_t != current_type.as_str() {
            panic!(
                "[ERROR] Wrong type for array element, expected `{}` found `{}`.",
                current_type[0..current_type.len() - 1].to_string(),
                t
            );
        }
    };

    // Parsing
    for ast in array.iter() {
        match ast {
            AST::ASTBool(v) => {
                check_type(&mut string, "array_bool");

                string += ";";
                string += v.to_string().as_str();
            }
            AST::ASTInt(v) => {
                check_type(&mut string, "array_int");

                string += ";";
                string += v.to_string().as_str();
            }
            AST::ASTString(v) => {
                check_type(&mut string, "array_string");

                string += ";";
                string += "'";
                string += v;
                string += "'";
            }
            AST::ASTArray(vec) => {
                let arr_string = convert_ast_array_to_string(vec, typecheck);

                if typecheck {
                    let pos = arr_string.find('#');
                    if let Some(n) = pos {
                        let mut t: String = String::from("array_");
                        t += &arr_string[0..n];
                        check_type(&mut string, t.as_str());
                    }
                }

                string += "|";
                string += arr_string.as_str();
                string += "|";
            }
            _ => {
                panic!("[ERROR] Invalid AST for conversion, found `{:?}`.", ast);
            }
        }
    }

    return string;
}

pub fn convert_ast_to_string(compound: &AST, typecheck: bool) -> TOMLStringMap {
    let mut map: TOMLStringMap = TOMLStringMap::new();

    let mut current_tag: String = String::from(".");

    if let AST::ASTCompound(asts) = &compound {
        for ast in asts.iter() {
            match ast {
                AST::ASTTagDefinition(n) => {
                    map.add_tag(n.as_str(), true);
                    current_tag = n.clone();
                }
                AST::ASTVariableDefinition(n, v) => {
                    let val: String = match v.as_ref() {
                        AST::ASTBool(v) => {
                            let mut str: String;

                            if typecheck {
                                str = String::from("bool#");
                            } else {
                                str = String::new();
                            }

                            str += v.to_string().as_str();
                            str
                        }
                        AST::ASTInt(v) => {
                            let mut str: String;

                            if typecheck {
                                str = String::from("int#");
                            } else {
                                str = String::new();
                            }

                            str += v.to_string().as_str();
                            str
                        }
                        AST::ASTString(v) => {
                            let mut str: String;

                            if typecheck {
                                str = String::from("string#'");
                            } else {
                                str = String::from("'");
                            }

                            str += v.as_str();
                            str += "'";
                            str
                        }
                        AST::ASTArray(v) => convert_ast_array_to_string(v, typecheck),
                        _ => {
                            panic!(
                                "[ERROR] Unknown type for variable value, type was `{:?}`.",
                                v
                            );
                        }
                    };
                    map.add_value(current_tag.as_str(), n, val.as_str());
                }
                AST::ASTSeparator() => {
                    map.add_tag(".", false);
                    current_tag = String::from(".");
                }
                _ => {
                    panic!("[ERROR] Invalid AST for conversion, found `{:?}`.", ast);
                }
            }
        }
    } else {
        panic!("[ERROR] Expected AST Compound found `{:?}`.", compound);
    }

    return map;
}
