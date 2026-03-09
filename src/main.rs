use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess (1-10): ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 10 {
                    println!("Please type a number between 1 and 10!");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        let rand_num = rand::thread_rng().gen_range(1..=10);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("The secret number is: {}", rand_num);
    }
}
