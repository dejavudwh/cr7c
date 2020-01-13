mod commandline;

fn main() {
    let content = commandline::run();
    println!("{}", content.as_bytes()[1]);
}
