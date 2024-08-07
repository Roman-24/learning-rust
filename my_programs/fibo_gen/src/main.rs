use text_io::read;

fn calculate_fibonaci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return calculate_fibonaci(n-1) + calculate_fibonaci(n-2)
    }
}

fn main() {
    println!("Hello, welcome to the _n_ fibonaci generator!");

    println!("Input your _n_ for generation fibonaci sequence");

    let user_value: u32 = read!();

    let result: u32 = calculate_fibonaci(user_value);

    println!("result is: {result}");
}
