use std::{env, error::Error, fs::File, io::Read};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(vec: Vec<String>) -> Result<Self, &'static str> {
        if vec.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = vec[1].clone();
        let filename = vec[2].clone();
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if vec.len() > 3 {
            case_sensitive = match vec[3].parse() {
                Ok(value) => value,
                Err(_) => return Err("Incorrect usage of CASE_INSENSITIVE argument (either true of false)")
            };
        }

        return Ok(Config {query, filename, case_sensitive});
    }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            vec.push(line);
        }
    }

    return vec;
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut vec = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            vec.push(line);
        }
    }

    return vec;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if config.case_sensitive {
        for lines in search(&config.query, &content) {
            println!("{lines}");
        }
    } else {
        for lines in search_case_insensitive(&config.query, &content) {
            println!("{lines}");
        }
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn case_sensitive() {
        let query = "to";
        let content = "\
                            I’m Nobody! Who are you?
                            Are you Nobody too?
                            Then there’s a pair of us!
                            Don't tell! they'd advertise you know!

                            How dreary to be Somebody!
                            How public like a Frog
                            To tell one’s name the livelong June
                            To an admiring Bog!";

        let vec = search(query, content);

        for line in vec {
            println!("{line}");
        }
    }

    #[test]
    pub fn case_insensitive() {
        let query = "to";
        let content = "\
                            I’m Nobody! Who are you?
                            Are you Nobody too?
                            Then there’s a pair of us!
                            Don't tell! they'd advertise you know!

                            How dreary to be Somebody!
                            How public like a Frog
                            To tell one’s name the livelong June
                            To an admiring Bog!";

        let vec = search_case_insensitive(query, content);

        for line in vec {
            println!("{line}");
        }
    }
}