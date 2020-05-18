use std::io;
extern crate rand;

use rand::Rng;


fn main() {
    println!("Guess the number!");
    let mut rng = rand::thread_rng();

    let n2: u16 = rng.gen();

    println!("The secret number is: {}", n2);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
