use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config : Config = parse_args(&args);

    let contents = fs::read_to_string(config.file_path)
        .expect("should be able to read file");
    
    println!("{contents}");
}

fn parse_args(args: &[String]) -> Config {
    return Config { query: args[1].clone(), file_path: args[2].clone() };
}
