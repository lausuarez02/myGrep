use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(| err | {
        eprintln!("problem passing arguments: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = minigrep::run(config){
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}

