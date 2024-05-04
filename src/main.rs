use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Generating a random number...");

    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random) {
            Ordering::Equal => {
                println!("You win!!!!!");
                break;
            },
            Ordering::Greater => {
                println!("Too large");
            },
            Ordering::Less => {
                println!("Too small");
            }
        }
    }
}