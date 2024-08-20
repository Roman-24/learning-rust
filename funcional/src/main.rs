fn main() {
    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;

    // call closure and store the result in a variable
    let x = 5;
    let result = add_one(x);

    println!("Result = {}", result);
}
