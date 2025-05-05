use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}
fn main() {
    let s = User {
        username: String::from("Pratik"),
        password: String::from("1234"),
    };

    let serialized = serde_json::to_string(&s);
    match serialized {
        Ok(str) => println!("{}", str),
        Err(_) => println!("Error converting to string")
    }
}