fn main() {
    let my_first_initial = 'C';
    check(my_first_initial);

    let your_character = '🥔';
    check(your_character);
}

fn check(character: char) {
    if character.is_alphabetic() {
        println!("Alphabetical!");
    } else if character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
