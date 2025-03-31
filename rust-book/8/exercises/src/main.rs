use std::collections::HashMap;

#[derive(Debug)]
struct Result {
    median: i32,
    mode: i32,
}

fn median_and_mode(values: &Vec<i32>) -> Result {
    let mut values_copy = values.to_vec();

    // median is the value in the middle of the sorted values
    let mut median = 0;
    if values_copy.len() != 0 {
        values_copy.sort();
        median = values_copy[values_copy.len() / 2];
    }

    let mut values_count = HashMap::new();
    for key in values_copy {
        let count = values_count.entry(key).or_insert(0);
        *count += 1;
    }

    // mode is the value that appears the most times in the values
    let mut mode = 0;
    let mut highest_count = 0;
    for (key, value) in values_count {
        if value > highest_count {
            mode = key;
            highest_count = value;
        }
    }

    Result { median, mode }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn pig_latin_translator(str: &str) -> String {
    let mut pig_latin_sentence: Vec<String> = Vec::new();

    for word in str.split_whitespace() {
        let first_letter = word.chars().next().unwrap();

        if is_vowel(first_letter) {
            pig_latin_sentence.push(format!("{word}ay"));
        } else {
            let mut vowel_index = 0;
            for c in word.chars() {
                if is_vowel(c) {
                    break;
                }
                vowel_index += 1;
            }

            pig_latin_sentence.push(
                format!("{}{}ay", &word[vowel_index..], &word[..vowel_index])
            );
        }
    }

    pig_latin_sentence.join(" ")
}

fn main() {
    let values = vec![4, 2, 1, 3, 4];
    let result = median_and_mode(&values);

    println!("values: {values:?}");
    println!("median: {}", result.median);
    println!("mode: {}", result.mode);

    let sentence = String::from("the quick brown fox is jumping over a fence in the school gym");
    let pig_latin_sentence = pig_latin_translator(&sentence);
    dbg!(sentence);
    dbg!(pig_latin_sentence);
}
