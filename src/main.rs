use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    if args.len() < 3 {
        panic!("too few arguments")
    }

    println!("searching for {} in {}", query, filename);
    

    let mut data = String::new();
    let f = File::open(filename);

    let mut file = match f{
        Ok(file) => file,
        Err(e) => panic!("error found {}", e)
    };

    file.read_to_string(&mut data)
    .expect("unexpected error occured and unable to read the file");

    println!("\n{data}")
}
