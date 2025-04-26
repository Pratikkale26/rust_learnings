/* 
fn main() {
    let str = String::from("Pratik");
    let len = get_length(str);
    println!("{}", len);

    print!("{}", str);
}

fn get_length(str: String) -> usize {
    return str.len()  // here after returning the name was removed from the heap.. thats why cant use that in main fnx = error
}
*/

fn main() {
    let str = String::from("Pratik");

    let (str, len) = get_length(str);
    println!("{} {}", str, len);
}

fn get_length(str: String) -> (String, usize) {
    let len = str.len();
    return (str, len); // transferring the ownership back -- this we dont use we use "references" (will learn tomorrow)
}