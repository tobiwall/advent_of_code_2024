use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut count = 0;
    let mut count2 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let result = parts[0].parse::<i64>().expect("Failed to parse result");
        let numbers_str: Vec<&str> = parts[1].split_whitespace().collect();
        let mut numbers = Vec::new();
        let mut line_map: HashMap<i64, Vec<i32>> = HashMap::new();
        for number in numbers_str {
            numbers.push(number.parse::<i32>().unwrap());
        }
        line_map.insert(result, numbers);

        for (key, numbers) in &line_map {
            let result = find_combination(numbers, *key);
            if result == true {
                count += key;
                count2 += key;
            } else {
                let result = find_combination_with_concat(numbers, *key);
                if result == true {
                    count2 += key;
                }
            }
        }
    }
    println!("Task 1: {count:?}");
    println!("Task 2: {count2:?}");
}

fn find_combination(numbers: &[i32], key: i64) -> bool {
    fn evaluate_expressions(numbers: &[i32], idx: usize, current_value: i64) -> Vec<i64> {
        if idx == numbers.len() {
            return vec![current_value];
        }

        let current_num = numbers[idx] as i64;
        [
            evaluate_expressions(numbers, idx + 1, current_value + current_num),
            evaluate_expressions(numbers, idx + 1, current_value * current_num),
        ]
        .concat()
    }
    let possible_results = evaluate_expressions(numbers, 1, numbers[0] as i64);
    possible_results.contains(&key)
}

fn find_combination_with_concat(numbers: &[i32], key: i64) -> bool {
    fn evaluate_expressions(numbers: &[i32], idx: usize, current_value: i64) -> Vec<i64> {
        if idx == numbers.len() {
            return vec![current_value];
        }

        let num = numbers[idx] as i64;
        let mut results = Vec::new();

        for (_op, next_value) in [
            ('+', current_value + num),
            ('*', current_value * num),
            (
                '|',
                format!("{}{}", current_value, num)
                    .parse::<i64>()
                    .unwrap_or(current_value),
            ),
        ] {
            results.extend(evaluate_expressions(numbers, idx + 1, next_value));
        }

        results
    }

    let possible_results = evaluate_expressions(numbers, 1, numbers[0] as i64);

    possible_results.contains(&key)
}
