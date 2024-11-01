struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) ->  f64 {
        self.width * self.height
    }
}

fn main() {
    let mut buf: String = String::new();

    println!("Input value for width:");
    std::io::stdin().read_line(&mut buf).expect("Couldn't read line");
    let width: f64 = buf.trim().parse().expect("Invalid value");

    buf.clear();

    println!("Input value for height:");
    std::io::stdin().read_line(&mut buf).expect("Couldn't read line");
    let height: f64 = buf.trim().parse().expect("Invalid value");

    let rectangle_1 = Rectangle {
        width: width,
        height: height,
    };

    println!("The area is: {}", rectangle_1.area());
}
