// use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let start_rng = 1;
    let end_rng = 1000;

    println!("Guess the number between {start_rng} and {end_rng} (inclusive)!");

    let correct_answer = rand::random_range(start_rng..=end_rng);
    // println!("the correct answer is: {correct_answer}");

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(msg) => {
                println!("{msg}");
                continue;
            },
        };

        match guess.cmp(&correct_answer) {
            Ordering::Less => println!("{guess} is too low!"),
            Ordering::Equal => {
                println!("You win! The number is {correct_answer}");
                break;
            }
            Ordering::Greater => println!("{guess} is too high!"),
        }
    }
}
