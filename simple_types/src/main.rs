fn main() {
    //MUTABILITY
    let mut x = 10;
    println!("Mutable old [x] = {}", x);
    
    x = 20;
    println!("Mutable new [x] = {}", x);


    //SHADOWING
    let y = 30;
    println!("Original old [y] = {}", y);

    let y = "Forty";
    println!("Shadowed new [y] = {}", y);

    //CONSTANT
    const MAX_VALUE: u32 = 1_000_000;
    println!("Const = {}", MAX_VALUE);


    //DATA TYPES

    //INTEGERS
    let a = 98_100;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b'A';

    println!("Int numbers: {}, {}, {}, {}, {}", a, b, c, d, e);

    //FLOAT
    let f: f32 = 2.01;
    let g = 43.0010001;

    println!("Float numbers: {}, {}", f, g);

    //BOOLEAN
    let h = true;
    let i: bool = false;

    println!("Booleans: {}, {}", h, i);

    //CHAR
    let j = 'a';
    let k = '\u{9842}';
    println!("Chars: {}, {}", j, k);

    
    //COMPOUND TYPES

    //TUPLES
    let tup = ("hello", 3292);
    let (hello_string, number) = tup;
    let number = tup.1;
    println!("Tuple: {:?}", tup);

    //ARRAYS (fixed length | vector -> dynamic length)
    let status_codes = [200, 201, 404];
    let ok = status_codes[0];
    let bytes = [0; 8];
    println!("Arrays: {:?}, {:?}", status_codes, bytes);
}
