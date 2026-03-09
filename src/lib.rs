use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments")
        }
        let query = args[1].clone(); 
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(&config.filename);
    println!("searching for {} in {}", &config.query,  &config.filename);

    let mut file = match f{
        Ok(file) => file,
        Err(e) => panic!("{e}")
    };

    let mut contents = String :: new();
    
    file.read_to_string(&mut contents)?;


    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents);
    }
    else {
        search_case_insensitive(&config.query, &contents);
    };

    println!("{:?}", result);


    Ok(())
}

fn search_case_sensitive <'a>(q: &str, contents: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();

    for l in contents.lines() {
        if l.contains(&q){
            result.push(l.trim());
        }
    }

    result
}


fn search_case_insensitive <'a>(q: &str, contents: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();

    for l in contents.lines() {
        if l.to_lowercase().contains(&q.to_lowercase()){
            result.push(l.trim());
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "biblioteca";
        let contents = "\
        Text:
            donde esta la biblioteca?
            Mi amo T-bone
            La aragna discoteca
        ";
    
        assert_eq!(vec!["donde esta la biblioteca?"], search_case_insensitive(query, contents));

    }

    #[test]
    fn case_insensitive(){
        let query = "Biblioteca";
        let contents = "\
        Text:
            donde esta la biblioteca?
            Mi amo T-bone
            La aragna discoteca
        ";
    
        assert_eq!(vec!["donde esta la biblioteca?"], search_case_insensitive(query, contents));
    }


    #[test]
    fn case_sensitive(){
        let query = "t-bone";
        let contents = "\
        Text:
            donde esta la biblioteca?
            Mi amo T-bone
            La aragna discoteca
            t-bone test
        ";
    
        assert_eq!(vec!["t-bone test"], search_case_sensitive(query, contents));
    }
}