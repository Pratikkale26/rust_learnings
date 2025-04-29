fn main() {
    create_str();
}

fn create_str() {
    let name: String = String::from("Pratik kale");

    let name2: String = name;

    // println!("{}", name); // it will give errors: ownership rules - cant be 2 owners(after removing one other will be pointing to none and double clearing issue)
    println!("{}", name2);
}