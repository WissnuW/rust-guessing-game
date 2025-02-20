use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your number guees");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Stop here stupid your guess, not your name!!!");
                return;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small stupid"),
            Ordering::Greater => println!("Stupid too big!!!!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
