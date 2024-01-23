fn main() {
    print_hello();
    
    let sum = sum(10, 20);
    print_number(sum);

    let subtraction = subtraction(20, 10);
    print_number(subtraction);

    let isGreater = check_max_value(sum, subtraction);
    println!("{}", isGreater);
}

fn print_hello() {
    println!("hello from function!");
}

fn print_number(number: i32) {
    println!("Number: {}", number);
}

fn sum(x: i32, y: i32) -> i32 {
    println!("Calculating sum from function...");
    x + y
}

fn subtraction(x: i32, y: i32) -> i32 {
    println!("Calculating subtraction from function...");
    return x - y;
}

fn check_max_value(threshold: i32, value: i32) -> bool {
    if value > threshold {
        return true;
    } else {
        return  false;
    }
}