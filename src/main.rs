use colored::*;
use std::cmp::Ordering;
use std::io;
//test change
//test change 2
fn main() {
    println!("Guess the number!");

    println!("");

    let mut secret_number = fastrand::i32(1..100);

    loop {
        
        println!("Please input your guess");

        println!();


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!();
                println!("{}", "Too small!".red());
                println!();
            }
            Ordering::Greater => {
                println!();
                println!("{}", "Too large!".red());
                println!();
            }
            Ordering::Equal => {
                println!();
                println!("{}", "You win!".bright_green());
                println!();
                println!("Would you like to continue playing? y/n");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line.");
                match input.trim() {
                    "y" => {
                        println!();
                        secret_number = fastrand::i32(1..100);
                        println!("A new number has been generated!")
                    }
                    "n" => break,
                    _ => { /* default case */ }
                }
            }
        }
    }
}
