#[allow(unused)]

fn main() {
    /*
    OWNERSHIP
    Rules:
        1. Each value has a variable called it's owner
        2. There can only be one owner for a given value at a time
        3. When the owner goes out of scope, the value is dropped
     */

    {
        let static_name = "name"; // -> allocated in the stack
        let dynamic_name = String::from("name"); // allocated in the heap (dynamic memory allocation)
    } //variables automatically deallocated when the scope is over (no more owner)


    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}",s1); -> error: s1 was moved to s2 and can no loger be used (not clone nor shallow copy)


    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{}",s3); // now the original value can be used


    let x = 0;
    let y = x;
    println!("{}", x); //with simple types (integers, chars, etc) the values are copied by default, and not moved


    let s5 = String::from("hello");
    take_ownership(s5); 
    //println!("{}", s5); -> error:
    //take_ownership(s: String) does not return a string, so when the string is sent to the function, 
    //it becomes the 's5's owner, and the value is droppend once the scope of the function ends


    let mut s6 = String::from("hello");
    s6 = take_ownership_and_return(s6); 
    // ownership of s6 goes to the function, and then the function returns the string, giving ownership back to the original scope
    println!("{}", s6);


    let s7 = String::from("some string");
    borrow_value(&s7);
    println!("{}", s7);
    // when the value is passed as reference (&), the ownership remains with the original owner
    // the function can now access the original value using a reference to s7
    // references are immutable by default


    let mut s8 = String::from("some string");
    borrow_and_change(&mut s8);
    println!("{}", s8);
    // by passing a mutating reference, the value can be mutated by the function borrowing the value


    let mut s9 = String::from("hello");
    let r1 = &mut s9;
    //let r2 = &mut s9; -> error: a value can not be passed as mutable reference more than once at a given time
    // this prevents data racing at compile time

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10; // this is allowed, since these references are immutable, and can only read the values from s10


    let mut s11 = String::from("hello");
    let r1 = &s11;
    //let r2 = &mut s11; error: can not borrow as a mutable reference if value has already been borrowed as immutable
    // immutable references expect the underlying data not to change 
}

/*
fn return_empty_reference() -> &String { //error: 's' will be dropped once the function ends, so the return would be of an "empty" address
    let s = String::from("ok");

    return &s;
}
 */

fn take_ownership(s: String) {
    println!("I own {}", s);
}

fn take_ownership_and_return(s: String) -> String {
    println!("I took {} and gave back", s);
    return s;
}

fn borrow_value(s: &String) {
    println!("I borrowed {}", s);
}

fn borrow_and_change(s: &mut String) {
    s.push_str(", pushed string");
}
