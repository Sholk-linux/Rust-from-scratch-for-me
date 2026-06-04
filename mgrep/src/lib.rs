use std::{env, fs, error::Error};

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Не достаточно аргументов");
        }

        let file_path = args[1].clone();
        let query = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { file_path, query, ignore_case, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_type = if config.ignore_case {
        searchp(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in search_type {
        println!("{line}");
    }

    Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


pub fn searchp<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            searchp(query, contents)
        );
    }

    #[test]
    fn cyr_case_sensitive() {
        let query = "аст";
        let contents ="\
РАСТ:
безопастный, быстрый, продуктивный.
Возьми три.
Доверься мне.";
        assert_eq!(
            vec!["безопастный, быстрый, продуктивный."], 
            search(query, contents)
        );
    }

    #[test]
    fn cyr_case_insensitive() {
        let query = "аст";
        let contents ="\
РАСТ:
безопастный, быстрый, продуктивный.
Возьми три.
Доверься мне.";
        assert_eq!(
            vec!["РАСТ:", "безопастный, быстрый, продуктивный."], 
            searchp(query, contents)
        );
    }

}

