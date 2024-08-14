#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );

    println!("rectangle is {rectangle1:?}");

    println!("rectangle is {rectangle1:#?}");

    dbg!(&rectangle1);

    let squar = Rectangle::square(5);
    dbg!(&squar);
}
