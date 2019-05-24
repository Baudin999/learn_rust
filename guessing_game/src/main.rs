use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Please guess a number");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 11);

    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_n) => {
                let g: i32 = guess.trim().parse().unwrap();

                println!("You guessed: {}", guess);

                match g.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!!"),
                    Ordering::Greater => println!("Too big!!"),
                    Ordering::Equal => {
                        println!("You win!!");
                        break;
                    }
                }
            }
            Err(error) => {
                println!("Something went wrong: {}", error);
                break;
            }
        }
    }
}
