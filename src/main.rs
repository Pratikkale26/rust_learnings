fn main() {
    let str = String::from("Pratik");

    let (str, len) = get_length(str);
    println!("{} {}", str, len);
}

fn get_length(str: String) -> (String, usize) {
    let len = str.len();
    return (str, len); // transferring the ownership back
}