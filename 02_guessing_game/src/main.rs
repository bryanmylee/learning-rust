use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
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

        // Match expressions are the result of all blocks being values.
        // All blocks are values unless a `;` is added.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
