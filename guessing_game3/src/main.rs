use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Tochi's guessing game v3");
    println!("Please guess a number");

    
    let computer_number:u8 = rand::thread_rng().gen_range(1..=100);
    
    loop{
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Couldn't read input");
        
        let guess:u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=>{
                println!("Please type a number");
                continue;
            }
        };
        
        println!("You guesed {guess}");
        
        
        match guess.cmp(&computer_number) {
            Ordering::Less => {
                println!("Go Higher");
                continue;
            }

            Ordering::Equal => {
                println!("You're correct");
                break;
            }
            
            Ordering::Greater => {
                println!("Go Lower");
                continue;
            }
        }
        
    }



    
}
