extern crate minigrep;

use std::env;
use std::process;
use minigrep::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error encountered while parsing args: {}", err);
        process::exit(1)
    });
    
    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    };
}



