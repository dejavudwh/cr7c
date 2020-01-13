mod commandline;
use lex::lexer;

fn main() {
    let content = commandline::run();
    let lexer = lexer::Lexer::new(content);
    println!("{}", content.as_bytes()[1]);
}
