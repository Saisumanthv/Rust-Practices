use std::io;
// use colored::*;

fn main() {
        
    loop{
        let mut in_num = String::new();
        io::stdin().read_line(&mut in_num).expect("Cannot read your num!");

        let num:u32 = match in_num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter numbers only!");
                continue
            }
        };

        if check_if_prime(&num){
            println!("{} is a prime number!", num);
            break
        }else{
            println!("Not a prime number!");
            break
        }
    }
}

fn check_if_prime(num:&u32) -> bool{
    if *num <= 1{
        return false;
    }
    if *num <= 3 {
        return true;
    }
    if *num%2 == 0 || *num%3 == 0{
        return false;
    }
    let mut i = 5;
    while i*i <= *num{
        if *num%i == 0 || *num % (i+2) == 0{
            return false;
        }
        i += 6;
    }
    return true;
}