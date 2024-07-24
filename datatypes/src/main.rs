fn main() {
    let x: i8 = 10;       // integers
    println!("{}", x);

    let _y: u8 = 10;

    let byte = b"A";
    println!("{:?}", byte);  // bytes

    let moa = b"MOA";
    println!("{:?}", moa);

    let moa = &b"MOA"[..];
    println!("{:?}", moa);

    let _a = 1.0;           // float
    let _b: f32 = 2.0;  

    let _t = true;           // bool 
    let _f: bool = false;

    let c = 'a';                // char
    println!("{}", c);
}
