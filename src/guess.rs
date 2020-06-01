#![allow(dead_code)]

use std::cmp::Ordering;
use std::io::stdin;

use rand::Rng;

fn guess_num() {
    let num = rand::thread_rng().gen_range(0, 100);
    println!("Guess the number!");
    loop {
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guess!");
        let guess_num = match guess.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        match guess_num.cmp(&num) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
