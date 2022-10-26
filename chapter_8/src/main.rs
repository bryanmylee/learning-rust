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
}

fn get_mean(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for number in numbers {
        sum += number
    }
    (sum as f32) / (numbers.len() as f32)
}

fn get_median(numbers: &Vec<i32>) -> f32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    if numbers.len() % 2 == 0 {
        let idx = numbers.len() / 2;
        ((numbers[idx] + numbers[idx - 1]) as f32) / 2.0
    } else {
        let idx = numbers.len() / 2;
        numbers[idx] as f32
    }
}

fn get_mode(numbers: &Vec<i32>) -> Vec<i32> {
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
