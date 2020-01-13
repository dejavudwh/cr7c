use std::fs;
use std::env;
use std::process;

struct Config {
    filename: String,
    // pub query: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments")
        }

        let filename = args[1].clone();

        Ok(Config {
            filename,
        })
    }
}

fn readfile(config: Config) -> String {
    let content = fs::read_to_string(config.filename).unwrap_or_else(|err| {
        println!("File read failed: {}", err);
        process::exit(1);
    });

    content
}

pub fn run() -> String {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let content = readfile(config);

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let args: Vec<String> = vec![String::from("a1"), String::from("a2"), String::from("a3")];

        let config = Config::new(&args).unwrap();
        assert_eq!("a2", config.filename);
    }
}