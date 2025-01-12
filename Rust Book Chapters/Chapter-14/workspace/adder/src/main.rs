extern crate add_one;
extern crate add_two;

fn main() {
    let num1 = 10;
    let num2 = 15;
    println!("{} + 1 = {}", num1, add_one::add_one(num1));
    println!("{} + 2 = {}", num2, add_two::add_two(num2));
}
