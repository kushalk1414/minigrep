use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    
    println!("searching for {} in {}", query, filename);
    
    
    let mut data = String::new();
    let f = File::open(filename);
    
    let mut file = match f{
        Ok(file) => file,
        Err(e) => panic!("{e}")
    };
    
    file.read_to_string(&mut data)
    .expect("unexpected error occured and unable to read the file");

println!("\n{data}")
}

fn parse_config(args: &[String]) -> (&str, &str) {
    if args.len() < 3 {
        panic!("too few arguments")
    }
    let query = &args[1];
    let filename = &args[2];
    println!("searching for {} in {}", query, filename);
    (query, filename)
}
