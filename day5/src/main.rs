use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let (rules, updates) = parse_input(&lines);

    let correctly_ordered_updates = count_correctly_ordered_updates(&rules, &updates);

    let sum_of_middle_page_numbers: i32 = correctly_ordered_updates
        .iter()
        .map(|update| get_middle_page_number(update))
        .sum();

    println!("Sum of middle page numbers: {}", sum_of_middle_page_numbers);
}

fn parse_input(lines: &[String]) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    
    for line in &lines[0..1176] {
        if let Some(pos) = line.find('|') {
            let left: i32 = line[0..pos].trim().parse().unwrap();
            let right: i32 = line[pos+1..].trim().parse().unwrap();
            rules.push((left, right));
        }
    }

    for line in &lines[1177..] {
        let update: Vec<i32> = line.split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        updates.push(update);
    }

    (rules, updates)
}

fn count_correctly_ordered_updates(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut correctly_ordered_updates = Vec::new();

    for update in updates {
        if is_correctly_ordered(update, rules) {
            correctly_ordered_updates.push(update.clone());
        }
    }

    correctly_ordered_updates
}

fn is_correctly_ordered(update: &[i32], rules: &[(i32, i32)]) -> bool {
    for &(left, right) in rules {
        if update.contains(&left) && update.contains(&right) {
            let left_index = update.iter().position(|&x| x == left).unwrap();
            let right_index = update.iter().position(|&x| x == right).unwrap();
            if left_index > right_index {
                return false;
            }
        }
    }
    true
}

fn get_middle_page_number(update: &[i32]) -> i32 {
    let len = update.len();
    let mid_index = len / 2;
    update[mid_index]
}
