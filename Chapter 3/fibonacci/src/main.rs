use std::{io, process::exit};

fn main() {
    println!("Fibonacci calculator - Finds the nth number of the fibonacci sequence
            \nInput a value for n:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Couldn't read input");

    let n : u8 = n.trim().parse()
                    .expect("Invalid number");

    if n == 0 {
        println!("Invalid number");
        exit(0);
    }

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u8) -> i64 {
    if n == 1 {
        return 0;
    } else if n == 2
    {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}