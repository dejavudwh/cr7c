mod commandline;
use lex::lexer;
use lex::token_str::TokenStr;

fn main() {
    let content = commandline::run();
    let lexer = lexer::Lexer::new(content);

    let v1: Vec<char> = vec!['a', 'b', 'c'];
    let v2: Vec<char> = vec!['a', 'b', 'v'];

    let t1 = TokenStr::new(v1);
    let t2 = TokenStr::new(v2);
    println!("{} {}", t1 == t2, "asd" == "asd");
}
