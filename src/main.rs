use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rand_number: u8 = rng.gen_range(1..100);

    println!("Start guessing a number between 1 and 99");
    loop {
        let guess = guess_number();
        if guess < rand_number {
            println!("Higher!");
        } else if guess > rand_number {
            println!("Lower!");
        } else {
            break;
        }
    }

    println!("You did it!");
}

fn guess_number() -> u8 {
    println!("Guess a number:");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let guess: u8 = input_line.trim().parse().expect("Input not a valid unsigned integer");
    guess
}