use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input your Guess...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    println! ("You guessed: {guess}");

}
