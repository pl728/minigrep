use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;
use minigrep::search_lowercase;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config { query, file_path, ignore_case });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    
    let results = if config.ignore_case {
        search_lowercase(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for r in results {
        println!("{r}");
    }

    Ok(())
}
