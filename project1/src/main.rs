mod test;

use test::printer;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number.");
    println!("Please input your guess.");

    printer::print("Jeanjean");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You don't input a number... try again :)");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        };
    }

    println!("Thanks for playing :D !");
}
