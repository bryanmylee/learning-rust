use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 3, 5, 6, 6, 7, 8, 10, 4, 8];
    println!(
        "{:?}, mean: {}, median: {}, mode: {:?}",
        numbers,
        get_mean(&numbers),
        get_median(&numbers),
        get_mode(&numbers)
    );

    let sentence = String::from("first apple");
    println!("{} in pig latin is {}.", sentence, pig_latin(&sentence));
}

fn get_mean(numbers: &[i32]) -> f32 {
    let mut sum = 0;
    for number in numbers {
        sum += number
    }
    (sum as f32) / (numbers.len() as f32)
}

fn get_median(numbers: &[i32]) -> f32 {
    let mut mut_numbers = vec![0; numbers.len()];
    numbers.clone_into(&mut mut_numbers);
    mut_numbers.sort();
    let numbers = mut_numbers;
    if numbers.len() % 2 == 0 {
        let idx = numbers.len() / 2;
        ((numbers[idx] + numbers[idx - 1]) as f32) / 2.0
    } else {
        let idx = numbers.len() / 2;
        numbers[idx] as f32
    }
}

fn get_mode(numbers: &[i32]) -> Vec<i32> {
    let mut number_to_count: HashMap<i32, i32> = HashMap::new();
    for number in numbers {
        let count = number_to_count.entry(*number).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    for count in number_to_count.values() {
        if *count > max_count {
            max_count = *count;
        }
    }
    let mut mode: Vec<i32> = vec![];
    for (number, count) in number_to_count {
        if count == max_count {
            mode.push(number);
        }
    }
    mode
}

fn pig_latin(sentence: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    for word in sentence.split_whitespace() {
        result.push(pig_latin_word(word));
    }
    result.join(" ")
}

fn pig_latin_word(word: &str) -> String {
    let mut chars = word.chars();
    let first = chars.next();
    if let Some(first) = first {
        if ['a', 'e', 'i', 'o', 'u'].contains(&first) {
            pig_latin_vowel(word)
        } else {
            pig_latin_consonant(first, chars.as_str())
        }
    } else {
        String::from("")
    }
}

fn pig_latin_consonant(first: char, rest: &str) -> String {
    let mut result = String::from(rest);
    result.push('-');
    result.push(first);
    result.push_str("ay");
    result
}

fn pig_latin_vowel(word: &str) -> String {
    let mut result = String::from(word);
    result.push_str("-hay");
    result
}
