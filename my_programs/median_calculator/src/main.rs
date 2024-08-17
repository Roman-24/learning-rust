fn calculate_median(mut number_collection: Vec<u32>) -> f32 {
    let n = number_collection.len();
    number_collection.sort();

    if n % 2 == 0 {
        let mid = n / 2;
        let median = (number_collection[mid - 1] + number_collection[mid]) as f32 / 2.0;
        median
    } else {
        let mid = n / 2;
        number_collection[mid] as f32
    }
}

fn main() {
    println!("Median calculator!");

    let input = vec![1, 2, 3, 4, 5];
    let result = calculate_median(input);
    println!("Result of given input is: {}", result);

    let input = vec![1, 2, 3, 4, 5, 6];
    let result = calculate_median(input);
    println!("Result of given input is: {}", result);
}
