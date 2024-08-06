use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("random number: {}", secret_number);
    
    println!("Please input your number");

    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("{}", "please input numbers only!".bright_yellow());
                continue
            },
        };
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too Small!".bright_red()),
            Ordering::Greater => println!("{}", "Too Big!".bright_red()),
            Ordering::Equal => {
                println!("{}", "You Win!".bright_green());
                break
            },
        };    
    }  
}

