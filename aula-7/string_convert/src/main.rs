fn main() {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).expect("Couldn't read line");
    let mut buf = buf.trim().to_string().to_lowercase();

    pig_latin(&mut buf);
    println!("{}", buf);
}

fn pig_latin(string: &mut String) {
    let vowels = ['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
    let first_char = string.trim().chars().next().unwrap();
    for vowel in vowels {
        if first_char == vowel {
            string.push_str("hay");
            return;
        }
    }

    let consonant = string.remove(0);
    string.push_str(&format!("{}ay", consonant)[..]);
}