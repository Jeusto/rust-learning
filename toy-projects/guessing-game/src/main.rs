use rand::Rng;
use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive range >= 1 and <= 100
    println!("{secret_number}");
    let mut guess = String::new();

    loop {
        guess.clear(); // to reuse the same String object
        println!("Please input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // println!("You guessed: {}", guess);
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("Correct guess");
                break;
            }
        }

        // if guess < secret_number {
        //     println!("Too small!");
        // } else if guess > secret_number {
        //     println!("Too big!");
        // } else {
        //     println!("Correct guess");
        //     break;
        // }
    }
}
