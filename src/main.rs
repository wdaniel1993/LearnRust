use std::io;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum ComparisonResult<'a> {
    Miss(&'a str),
    Hit,
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_number: u8 = rng.gen_range(1..100);

    println!("Start guessing a number between 1 and 99");
    loop {
        let guess = guess_number();
        match compare_numbers(guess, rand_number) {
            ComparisonResult::Hit => break,
            ComparisonResult::Miss(result) => println!("{}", result)
        }
    }

    println!("You did it!");
}

fn compare_numbers<'a>(guess: u8, rand_number: u8) -> ComparisonResult<'a>{
    if guess < rand_number {
        return ComparisonResult::Miss("Higher!");
    } else if guess > rand_number {
        return ComparisonResult::Miss("Lower!");
    } else {
        return ComparisonResult::Hit;
    }
}

fn guess_number() -> u8 {
    println!("Guess a number:");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let guess: u8 = input_line.trim().parse().expect("Input not a valid unsigned integer");
    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_items_equal_item_hit() {
        assert_eq!(compare_numbers(4,4), ComparisonResult::Hit);
    }

    #[test]
    fn compare_items_smaller_item_miss() {
        assert_eq!(compare_numbers(4,5), ComparisonResult::Miss("Higher!"));
    }

    #[test]
    fn compare_items_bigger_item_miss() {
        assert_eq!(compare_numbers(5,4), ComparisonResult::Miss("Lower!"));
    }
}