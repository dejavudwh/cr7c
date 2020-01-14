mod commandline;
use lex::lexer;

fn main() {
    let content = commandline::run();
    let mut lexer = lexer::Lexer::new(content);

    for i in 0..5 {
        println!("{}", lexer.lex());
    }
}
