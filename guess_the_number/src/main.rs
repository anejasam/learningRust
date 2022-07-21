use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");
    println!("Please input your guess");

    let random = rand::thread_rng().gen_range(0..=100);

    loop { 
        let mut guess = String::new();
       
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed {guess}");
        
        match guess.cmp(&random) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    } 
}
