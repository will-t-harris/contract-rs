use std::env;
use std::process;

use contract::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        
        process::exit(1);
    });

    
    

    if let Err(e) = contract::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
