use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pos(i32, i32);

#[derive(Debug, PartialEq)]
struct OneChar {
    char: char,
    pos: Pos,
}

#[derive(Debug)]
struct Distance(i32, i32);

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut char_single: Vec<char> = Vec::new();
    let mut all_chars: Vec<OneChar> = Vec::new();
    get_all_chars(&mut char_single, &mut all_chars, lines);
    check_each_char(char_single, all_chars);
}

fn get_all_chars(char_single: &mut Vec<char>, all_chars: &mut Vec<OneChar>, lines: Vec<String>) {
    let mut row = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            if find_chars(c) {
                let char = OneChar {
                    char: c,
                    pos: Pos(row, x),
                };
                all_chars.push(char);
                if !char_single.contains(&c) {
                    char_single.push(c);
                }
            }
            x += 1;
        }
        row += 1;
    }
}

fn find_chars(char: char) -> bool {
    char != '.'
}

fn check_each_char(char_single: Vec<char>, all_chars: Vec<OneChar>) {
    let mut count = 0;
    let mut locations = Vec::new();
    for char in char_single {
        let same_chars: Vec<&OneChar> = all_chars.iter().filter(|c| c.char == char).collect();
        for next_char in &same_chars {
            for second_char in &same_chars {
                if next_char != second_char {
                    let distance = check_distance(next_char, second_char);
                    let x = second_char.pos.0 + distance.0;
                    let y = second_char.pos.1 + distance.1;
                    let location = Pos(x, y);
                    if location.0 <= 49 && location.0 >= 0 && location.1 <= 49 && location.1 >= 0 {
                        if !locations.contains(&location) {
                            count += 1;
                        }
                    }
                    locations.push(location.clone());
                }
            }
        }
    }
    println!("{count}");
}

fn check_distance(first: &OneChar, next: &OneChar) -> Distance {
    let dx = next.pos.0 - first.pos.0;
    let dy = next.pos.1 - first.pos.1;

    Distance(dx, dy)
}
