fn five() -> i32 {
    std::f64::consts::PI as i32
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);

    let y = plus_one(x);

    println!("The value of y is: {}", y);
}