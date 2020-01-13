mod commandline;

fn main() {
    let content = commandline::run();
    println!("{}", content);
}
