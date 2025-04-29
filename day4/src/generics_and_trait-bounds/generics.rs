fn main() {
    let s= sum(5, 7);
    println!("{}", s);
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b:T) -> T {
    return a + b;
}