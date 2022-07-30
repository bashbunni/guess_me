use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // give us a random number generator, then gen a new rand int
    let secret_number = rand::thread_rng().gen_range(1..=2);

let mut guess = String::new();
    loop {
        guess.clear();
        println!("Please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            _ => continue,
        };

        println!("you guessed: {guess}");
        let is_match = matches(guess_int, secret_number);
        if is_match { break };
    }
}

fn matches(guess: u32, secret_number: u32) -> bool {
    match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                false
            }
            Ordering::Greater => { 
                println!("Too big");
                false
            }
            Ordering::Equal => { 
                println!("you get a gold star");
                true
            }
        // if we didn't need feedback
        // Ordering::Equal => true, _ => false
    }
}
