fn main() {
    // let x = 5; // is immutable
    let mut x = 7; // is mutable 
    
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// by default everything is immutable