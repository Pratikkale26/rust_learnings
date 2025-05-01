use std::fmt::{Debug, Display, Formatter};

// #[derive(Debug)]         // use this to avoid writing below trait impl for the struct
struct User {
	username: String,
	password: String,
	age: u32
}

// to avoid writting this much code (trait impl) - use procedural macro = #[derive(Debug)]
impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.username, self.password, self.age)
    }
}

// we dont have display macro - we need to derive (impl trait) it everytime we need to disply something to the end user (ofcourse security reasons)
impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.username, self.password, self.age)
    }
}

fn main() {
    let user = User {
        username: String::from("Pratik"),
        password: String::from("kalep"),
        age: 19
    };
    println!("{:?}", user);
    println!("{}", user)
}