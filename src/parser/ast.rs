#[derive(Debug)]
pub enum AST {
    ASTTagDefinition(String),
    ASTVariableDefinition(String, Box<AST>),
    ASTSeparator(),
    ASTBool(bool),
    ASTInt(i32),
    ASTString(String),
    ASTArray(Vec<AST>),
    ASTCompound(Vec<AST>),
}
