fn main() {
    //Rust will infer what type want use
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess: {guess}");

    //Integer Types
    // i8, i16, i32, i64, i128, isize: Signed 
    // u8, u16, u32, u64, u128. usize: Unsigned

    //Integer Literals in Rust
    //Decimal: 98_22
    //Hex: 0xff
    //Octal: 0o77
    //Binary: 0b1111_0000
    //Byte (u8 only): b'A'

    //Floating-point Types
    let x = 2.0; //f64 default
    let y: f32 = 3.0; //f32

    //Numeric Operations
    //addition 
    let sum = 5 + 10;
    
    //subtraction
    let difference = 6.2 - 3.2;

    //multiplication
    let product = 4 * 10;

    //division
    let quotient = 51.6 / 13.7;
    let floored = 2/3; //Result in 0

    //remainder
    let remainder = 43 % 5;

    //The Boolean Type
    let t = true;
    let f: bool = false; 

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';


    //Compound Types
    
    //The Tuple Type: fixed length, once declared, can't grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //The Array Type: Same type and fixed length
    let a = [1, 2, 3, 4, 5];

    // i32 is the type and 5 elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; //result a = [3, 3, 3, 3, 3];

    //Accressing Array Elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    
}
