#[derive(Clone, Debug, PartialEq)]
pub struct ProgramNode {
    pub import_stmts: Vec<ImportStmtNode>,
    pub defs: Vec<TopDefNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportStmtNode {
    pub paths: Vec<String>,
}

// impl fmt::Display for ImportStmtNode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.paths)
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct TopDefNode {

}