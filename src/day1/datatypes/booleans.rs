fn main() {
    println!("{}", is_even(6));
}

fn is_even(a: u32) -> bool {
    return a % 2 == 0;
}
