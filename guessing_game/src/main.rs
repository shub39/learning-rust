use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number");

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too Small");
                tries += 1;
            }
            Ordering::Greater => {
                println!("Too Big");
                tries += 1;
            }
            Ordering::Equal => {
                println!("You Win!! in {tries} tries.");
                break;
            }
        }
    }
}
