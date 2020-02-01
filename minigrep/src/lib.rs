use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // cp. 12
    //pub fn new(args: &[String]) -> Result<Config, &'static str> {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        
        // cp. 12
        // if args.len() < 3 {
        //     return Err("필요 인수 부족")
        // }
        //
        // let query = args[1].clone();
        // let filename = args[2].clone();

        match args.next() {
            Some(args) => println!("name {}",args),
            None => (),
        }; //path

        let query = match args.next() {
            Some(args) => args,
            None => return Err("쿼리 음슴"),
        };
        let filename = match args.next() {
            Some(args) => args,
            None => return Err("파일명 음슴"),
        };


        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query,&contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    println!("results :\n{:?}", results);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()

    // cp. 12
    // let mut results = Vec::new();
    
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }

    // }
    // results
    
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
    // cp. 12
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line)
    //     }

    // }
    // results
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    
    assert_eq!(vec!["safe, fast, productive."],search(query, contents));

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    
    assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, contents));

    }
}