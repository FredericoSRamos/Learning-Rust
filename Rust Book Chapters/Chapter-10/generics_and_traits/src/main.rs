use std::fmt::Display;

use generics_and_traits::Test;

struct Announcement {
    message: String,
}

impl Test for Announcement {
    fn test(&self) {
        if self.message.len() < 10 {
            println!("Message passed the test!");
        } else {
            println!("Message did not pass the test")
        }
    }
}

impl Display for Announcement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn main() {
    let x = "Hello";
    let y = "Test";

    let ann = Announcement {
        message: "This is an announcement".to_string(),
    };
    
    let string = longest_with_an_announcement(x, y, ann).to_string();
    println!("{}", string);
}

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str 
where 
    T: Display + Test
{
    println!("Announcement: {}", ann);

    if x.len() > y.len() {
        return x;
    }

    return y;
}
