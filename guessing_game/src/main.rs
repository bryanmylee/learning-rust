use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Declare a new variable with `let`.
    // Mark the variable as mutable with `mut`.
    // Assign a new string instance to the variable.
    let mut guess = String::new();

    io::stdin()
        // Pass a mutable reference to `guess`.
        // The & indicates that this argument is a reference.
        .read_line(&mut guess)
        // Handle the Result type returned from `.read_line()`.
        .expect("Failed to read line");

    println!("You guessed: {}", guess)
}
