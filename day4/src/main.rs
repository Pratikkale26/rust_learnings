use std::f32::consts::PI;

// define trait -- its like interface, characteristics
trait Shape {
    fn area(&self) -> f32;
}

struct Rect {
    width: f32,
    height: f32,
}

impl Shape for Rect {  // here we impl Shape trait for Rect
    fn area(&self) -> f32 {
        return self.width * self.height
    }
}

struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius
    }
}

fn get_area<T: Shape>(shape: T)  {      // here get_area is a generic function
    println!("The area is {}", shape.area());
}

fn main() {

    let c = Circle {
        radius: 10.0
    };
    
    let r = Rect {
        width: 10.0,
        height: 20.0,
    };
    
    get_area(r);
    get_area(c);
}


