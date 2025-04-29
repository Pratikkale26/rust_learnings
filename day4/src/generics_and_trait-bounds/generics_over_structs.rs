struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    pub fn area(&self) -> T {
        return self.height * self.width
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20
    };

    println!("{}", r.area());
}