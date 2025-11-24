use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}\n", config.file_path);

    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = search(&config.query, &contents);
    for r in results {
        println!("{r}");
    }

    Ok(())
}
