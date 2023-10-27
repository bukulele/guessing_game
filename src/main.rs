use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=== GUESS THE NUMBER GAME ===");

    const MIN_NUMBER: i64 = 1;
    const MAX_NUMBER: i64 = 100;

    let mut rng = rand::thread_rng();
    let secret_number: i64 = rng.gen_range(MIN_NUMBER..=MAX_NUMBER);
    println!(
        "**COMPUTER: I'm thinking of a number from {MIN_NUMBER} to {MAX_NUMBER}. Try to guess it!"
    );

    let mut number_of_guesses: u32 = 0;

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number in range from {MIN_NUMBER} to {MAX_NUMBER}.");
                continue;
            }
        };

        if (guess < MIN_NUMBER) | (guess > MAX_NUMBER) {
            println!("Please type a number in range from {MIN_NUMBER} to {MAX_NUMBER}.");
            continue;
        }

        number_of_guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Equal => break println!("Yes! The correct number is {secret_number}. Your number of tries is {number_of_guesses}"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
