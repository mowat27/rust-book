use std::collections::HashMap;

struct Averages {
    mean: f32,
    median: i32,
    mode: i32,
}

impl Averages {
    fn calculate(numbers: &Vec<i32>) -> Averages {
        Averages {
            mean: mean(&numbers),     // 4.8
            median: median(&numbers), // 4
            mode: mode(&numbers),     // 4
        }
    }

    fn print(&self) {
        println!(
            "mean={:.2}, median={}, mode = {}",
            self.mean, self.median, self.mode
        );
    }
}

fn main() {
    let numbers = vec![3, 7, 5, 7, 4, 1, 7, 9, 7, 3, 4, 1, 1, 1, 1];
    let averages = Averages::calculate(&numbers);
    println!("Averages of {:?}: ", numbers);
    averages.print();
}

fn mean(numbers: &Vec<i32>) -> f32 {
    let total: i32 = numbers.iter().sum();
    total as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    numbers[numbers.len() / 2]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let (mut max, mut result) = (0, 0);
    for (num, count) in count_occurrences(&numbers) {
        if count > max {
            max = count;
            result = num;
        }
    }
    result
}

fn count_occurrences(numbers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for &n in numbers.iter() {
        *counts.entry(n).or_insert(0) += 1;
    }
    counts
}
