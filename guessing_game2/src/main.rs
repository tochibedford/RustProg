use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number");

    println!("Please input your guess: ");
    
    
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to ReadLine");
        
        let guess: u8 = match guess.trim().parse() {
            Result::Ok(num)=>num,
            Err(_) => {
                println!("please type a number");
                continue;
            }
        };

        println!("You guessed: {guess}");


        println!("Actual Number {random_number}");
        
        match guess.cmp(&random_number){
            Ordering::Less => {
                println!("Too low");
            }
            Ordering::Equal => {
                println!("You're Correct");
                break;
            }
            Ordering::Greater => {
                println!("Too high");
            }
        }
    }
}
