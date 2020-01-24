use lex::lexer::Lexer;
use lex::token::Token;
use crate::ast:: {
    ProgramNode,
};
use crate::parse_def:: {
    import_stmts,
    top_defs,
};

// pub fn parse(mut lexer: &mut Lexer) {
//     compilation_unit(&mut lexer);
// }

// fn compilation_unit(mut lexer: &mut Lexer) -> ProgramNode {
//         // import_stmts + defs + EOF
//         let mut node = ProgramNode {
//             import_stmts: Vec::new(),
//             defs: Vec::new(),
//         };
//         let token = lexer.lookahead(1);
//         match token {
//             Token::Import => node.import_stmts = import_stmts(&mut lexer),
//             Token::Eof => (),
//             _ => node.defs = top_defs(&mut lexer),
//         };

//         return node
// }