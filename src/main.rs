use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Self {
        let q = args[1].clone();
        let f = args[2].clone();
        return Config { query: q, file_path: f };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config : Config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path)
        .expect("should be able to read file");
    
    println!("{contents}");
}
