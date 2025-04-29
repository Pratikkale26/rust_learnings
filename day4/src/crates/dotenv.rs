use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("name");

    match var {
        Ok(name) => println!("Hello, {}!", name),
        Err(_e) => println!("Hello, world!"),
    }
}
