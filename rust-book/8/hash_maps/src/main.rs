use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    scores.entry(String::from("blue")).or_insert(100);

    for (key, value) in &scores {
        println!("{key} team score is {value}");
    }

    // let team_name = String::from("blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    let mut letter_count = HashMap::new();

    for letter in text.chars() {
        let count = letter_count.entry(letter).or_insert(0);
        *count += 1;
    }

    println!("{word_count:?}");
    println!("{letter_count:?}");
}
