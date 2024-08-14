#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle1)
    );

    println!("rectangle is {rectangle1:?}");

    println!("rectangle is {rectangle1:#?}");

    dbg!(&rectangle1);
}
