fn main() {
    let mut counter = 0;

    let i = loop {
        counter += 1;

        if counter >= 10 {
            break counter;
        };

        println!("Loop: {}", counter);
    };

    println!("Loop counter: {}", i);

    counter = 0;

    while counter < 10 {
        counter += 1;
        println!("While: {}", counter);
    };

    let seq = [0,1,2,3,4,5];

    for item in seq.iter() {
        println!("For (iter): {}", item);
    }

    for item in 0..10 {
        println!("For (range): {}", item);
    }
}
