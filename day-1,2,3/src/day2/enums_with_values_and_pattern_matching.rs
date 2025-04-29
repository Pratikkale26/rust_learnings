/*
// example: 2 (enum with associated data)

use std::f64::consts::PI;

// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape 

    if let Shape::Circle(radius) = shape {
        return std::f64::consts::PI * radius * radius;
    }
    if let Shape::Square(side) = shape {
        return side * side;
    }
    if let Shape::Rectangle(width, height) = shape {
        return width * height;
    }
    return 0.0;

}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    
    println!("The area of the circle is {}", calculate_area(circle));
    println!("The area of the square is {}", calculate_area(square));    
    println!("The area of the rectangle is {}", calculate_area(rectangle));
}

 */

// example: 3 (enum with associated data and pattern matching)

use std::f64::consts::PI;

enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height
        }
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("The area of the circle is {}", circle.area());
    println!("The area of the square is {}", square.area());    
    println!("The area of the rectangle is {}", rectangle.area());
}
