fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score of team {} is {}", team_name, score);

    // iteration over hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // only inserting if the key has no value
    scores.entry(String::from("Blue")).or_insert(420);
    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
