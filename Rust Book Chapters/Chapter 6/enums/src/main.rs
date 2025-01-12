use std::io;

const BUFFER_SIZE: usize = 80;

enum ArrayControl {
    Read,
    Write([char; BUFFER_SIZE])
}

impl ArrayControl {
    fn read(&self, message: &String, array: &mut [char]) {
        let mut track = 0;
        for chars in message.chars() {
            array[track] = chars;
            track += 1;
        }
    }

    fn write(&self, array: [char; BUFFER_SIZE]) {
        for chars in array {
            print!("{}", chars);
        }
    }
}

fn main() {
    let mut arr: [char; BUFFER_SIZE] = ['\0'; BUFFER_SIZE];
    let mut arr_control: ArrayControl = ArrayControl::Read;

    handle_control(arr_control, &mut arr);

    arr_control = ArrayControl::Write(arr.clone());

    handle_control(arr_control, &mut arr);
}

fn handle_control(arr_control: ArrayControl, arr: &mut [char]) {
    match arr_control {
        ArrayControl::Write(array) => {
            arr_control.write(array);
        }
        ArrayControl::Read => {
            let mut buf = String::new();

            println!("Input a message:");
            io::stdin().read_line(&mut buf).expect("Couldn't read line");

            arr_control.read(&mut buf, arr);
            buf.clear();
        }
    }
}
