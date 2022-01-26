use std::io;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Your secret number is {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        println!("Your guess is {}", guess);
    
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}","Too small".red()),
            std::cmp::Ordering::Greater => println!("{}","Too big".red()),
            std::cmp::Ordering::Equal => {
                println!("{}","You win".green());
                break; 
            }
        }
    }
   
}
