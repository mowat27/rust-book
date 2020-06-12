use std::collections::HashMap;

struct Averages {
    mean: f32,
    median: i32,
    mode: i32,
}

fn main() {
    let numbers = vec![3, 7, 5, 7, 4, 1, 7, 9, 7, 3, 4, 1, 1, 1, 1];
    let averages = Averages {
        mean: mean(&numbers),     // 4.8
        median: median(&numbers), // 4
        mode: mode(&numbers),     // 4
    };
    println!("\n -- Averages from {:?} --\n", numbers);
    println!("mean = {:.2}", averages.mean);
    println!("median = {}", averages.median);
    println!("mode = {}", averages.mode);
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let mut total = 0;
    for n in numbers.iter() {
        total += n;
    }
    total as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    numbers[numbers.len() / 2]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for n in numbers.iter() {
        let c = counts.entry(n).or_insert(0);
        *c += 1;
    }
    let mut result: (i32, i32) = (0, 0);
    for (&n, &c) in &counts {
        if &c > &result.1 {
            result = (*n, c);
        }
    }
    result.0
}
