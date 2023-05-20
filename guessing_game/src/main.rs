use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");
    loop {
        println!("Input the number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("too small"),
        }
    }
    // if secret_number = guess {
    //     println!("Congrats!!");
    // }
}
