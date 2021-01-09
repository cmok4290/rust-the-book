use std::io; // standard input/output library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable of String instance

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
