use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut guessed_correctly: bool = false;

    while guessed_correctly == false {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                guessed_correctly = true;
            },
            Ordering::Greater => println!("Your guess was too large!")
        }
    }
}
