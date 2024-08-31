fn main() {
    // print_phrase("Hello from Parameter");
    println!("{}", gcd(20,4));
    println!("{}", multiple_return_values(true));
}

// fn print_phrase(phrase: &str){
//     println!("{}", phrase);
// }

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0{
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn multiple_return_values(flag: bool) -> bool{
    if flag == true{
        true
    }else{
        false
    }
}