/*
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();
}
*/

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_function!(hello);  // This will create a function called "hello"

fn main() {
    hello();  // Prints "Hello from hello"
}