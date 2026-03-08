use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments")
        }
        let query = args[1].clone(); 
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(&config.filename);
    println!("searching for {} in {}", &config.query,  &config.filename);

    let mut file = match f{
        Ok(file) => file,
        Err(e) => panic!("{e}")
    };

    let mut data = String :: new();
    
    file.read_to_string(&mut data)?;


    println!("\n{data}");
    Ok(())
}