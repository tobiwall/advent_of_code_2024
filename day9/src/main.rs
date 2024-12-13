use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    let input = convert_vec(reader);
    let ordered_vec = order_blocks(input);
    let changed_vec = create_num_vec(ordered_vec);
    let part1_sum = sum(changed_vec);

    println!("Part1: {part1_sum}");
}

fn convert_vec(reader: BufReader<File>) -> Vec<i32> {
    let lines: Vec<i32> = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect::<Vec<i32>>()
        })
        .collect();
    lines
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Block {
    Number(i64),
    Dot,
}

fn order_blocks(input: Vec<i32>) -> Vec<Block> {
    let mut idx = 0;
    let mut vec: Vec<Block> = Vec::new();

    for i in (0..input.len()).step_by(2) {
        let mut first = input[i];
        let mut second = if i + 1 < input.len() { input[i + 1] } else { 0 };

        while first > 0 {
            vec.push(Block::Number(idx));
            first -= 1;
        }
        idx += 1;
        while second > 0 {
            vec.push(Block::Dot);
            second -= 1;
        }
    }
    vec
}

fn create_num_vec(mut input: Vec<Block>) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();

    for n in input.clone() {
        if n == Block::Dot {
            if let Some(last_num) = find_last_num(&mut input) {
                insert_at_first_dot(&mut input, last_num);
            }
        } else {
            continue;
        }
        match n {
            Block::Number(num) => {
                vec.push(num);
            }
            Block::Dot => continue,
        }
    }
    for n in input.clone() {
        match n {
            Block::Number(num) => {
                vec.push(num);
            }
            Block::Dot => continue,
        }
    }
    vec
}

fn find_last_num(input: &mut Vec<Block>) -> Option<Block> {
    if let Some(pos) = input
        .iter()
        .rposition(|block| matches!(block, Block::Number(_)))
    {
        Some(input.remove(pos))
    } else {
        None
    }
}

fn insert_at_first_dot(input: &mut Vec<Block>, block: Block) {
    if let Some(pos) = input.iter().position(|b| matches!(b, Block::Dot)) {
        input[pos] = block;
    }
}

fn sum(input: Vec<i64>) -> i64 {
    let mut i: i64 = 0;
    let mut sum: i64 = 0;
    for num in input {
        sum += num * i;
        i += 1;
    }
    sum
}
