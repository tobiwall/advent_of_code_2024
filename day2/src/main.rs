use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_input = read_file();
    check_each_line(file_input);
}

fn read_file() -> BufReader<File> {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    reader
}

fn check_each_line(file_input: BufReader<File>) {
    let mut sum_right_lines = 0;
    for line in file_input.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let consecute = check_consecutive(numbers.clone());
        if consecute == true {
            let right_difference = check_difference(numbers.clone());
            if right_difference == true {
                sum_right_lines += 1;
            }
        }
    }
    println!("It is {sum_right_lines}");
}

fn check_consecutive(numbers: Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            is_decreasing = false;
        } else if numbers[i] < numbers[i - 1] {
            is_increasing = false;
        }
    }
    is_increasing || is_decreasing
}

fn check_difference(numbers: Vec<i32>) -> bool {
    let mut right_difference = true;
    for i in 1..numbers.len() {
        if (numbers[i] - numbers[i - 1]).abs() > 3 {
            right_difference = false;
        }
        if (numbers[i] - numbers[i - 1]).abs() < 1 {
            right_difference = false;
        }
    }
    right_difference
}