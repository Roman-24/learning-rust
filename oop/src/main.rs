// Define a trait for Shape
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

// Implement the Shape trait for Circle
struct Circle {
    radius: f64,
}

impl Circle {
    // Constructor for Circle
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

// Implement the Shape trait for Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Constructor for Rectangle
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

// Function to print the area of any shape
fn print_area(shape: &dyn Shape) {
    println!("The area of the {} is {}", shape.name(), shape.area());
}

fn main() {
    let circle = Circle::new(3.0);
    let rectangle = Rectangle::new(4.0, 5.0);

    print_area(&circle);
    print_area(&rectangle);
}
