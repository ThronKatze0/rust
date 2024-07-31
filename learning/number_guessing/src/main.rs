use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Welcome to numbur guessing game!");
    let rand_number: i32 = rand::thread_rng().gen_range(0..=100);
    loop {
        let mut guess: String = String::new();
        println!("Give me your number: ");
        io::stdin().read_line(&mut guess).expect("IO Error");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.eq("exit") {
                    println!("yeah");
                    break;
                }
                println!("Invalid number!");
                continue;
            }
        };
        match guess.cmp(&rand_number) {
            cmp::Ordering::Less => println!("Wanted number is higher"),
            cmp::Ordering::Equal => {
                println!("You won!");
                break;
            }
            cmp::Ordering::Greater => println!("Wanted number is lower"),
        }
    }
}
