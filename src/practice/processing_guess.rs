use std::io;
fn main() {
    println!("guess a number");

    let mut guess=String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}