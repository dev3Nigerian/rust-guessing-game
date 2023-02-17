//import necessary crates
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //create a random secret number
    let secret_number = rand::thread_rng().gen_range(1..=20);

    // loops the User's input
    loop {
        println!("Between 1 and 20");

        let mut guess = String::new();

        //Takes in user input
        io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

        //Handle the error input
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println! ("You guessed: {guess}");

        //Matches the secret guess value to user's input
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    

}
