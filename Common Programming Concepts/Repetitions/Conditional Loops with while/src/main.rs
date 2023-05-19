use std::thread;
use std::time::Duration;

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        thread::sleep(Duration::from_secs(1));

        number -= 1;
    }

    println!("LIFTOFF!!!");
}