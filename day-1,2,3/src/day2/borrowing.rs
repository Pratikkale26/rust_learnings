// example: 1 - multiple immutable references (it will run)
/*
fn main() {
    let str = String::from("Pratik");
    let ref1 = &str;
    let ref2 = &str;

    println!("{} {}", ref1, ref2);
}
*/

// example: 2 - multiple mutable references (it will not run)
/*
fn main() {
    let mut str = String::from("Pratik");
    let ref1 = &mut str;
    let ref2 = &mut str;

    println!("{} {}", ref1, ref2);
}
*/

// example: 3 
fn main() {
    let mut str = String::from("Pratik ");
    let ref1 = &mut str;
    ref1.push_str("Kale");   // here after pushing, the borrowed return the ownership to the string - lifespan ends

    let ref2 = &str;    // thats why it will work
    
    println!("{}", ref2);
}


// Rules of borrowing -

// 1. You can only have one mutable reference. If there is an mutable reference, there cant be other immutable or mutable references
// 2. You can have multiple immutable references