fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }

    println!();

    let v2 = vec![1, 2, 3];
    for val in v2 {
        println!("Got: {val}");
    }
}
