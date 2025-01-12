use std::{env, error::Error, fs::File, io::Read};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(iter: &mut env::Args) -> Result<Self, &'static str> {
        iter.next();

        let query = match iter.next() {
            Some(value) => value,
            None => return Err("Not enough parameters")
        };

        let filename = match iter.next() {
            Some(value) => value,
            None => return Err("Not enough parameters")
        };

        let case_sensitive = match iter.next() {
            Some(value) => {
                match value.parse() {
                    Ok(result) => result,
                    Err(_) => {
                        return Err("Incorrect usage of CASE_INSENSITIVE argument (either true or false)")
                    }
                }
            }
            None => env::var("CASE_INSENSITIVE").is_err()
        };

        return Ok(Config {
            query,
            filename,
            case_sensitive
        });
    }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines().filter(|line| line.contains(query)).collect();
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect();
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