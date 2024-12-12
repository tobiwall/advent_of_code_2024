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
    let lines = BufReader::new(File::open("input.txt").expect("Failed to open the file"))
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut char_single = Vec::new();
    let mut all_chars = Vec::new();

    get_all_chars(&mut char_single, &mut all_chars, lines);
    check_each_char(char_single, all_chars);
}

fn get_all_chars(char_single: &mut Vec<char>, all_chars: &mut Vec<OneChar>, lines: Vec<String>) {
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                all_chars.push(OneChar {
                    char: c,
                    pos: Pos(row as i32, x as i32),
                });
                if !char_single.contains(&c) {
                    char_single.push(c);
                }
            }
        });
    });
}

fn check_each_char(char_single: Vec<char>, all_chars: Vec<OneChar>) {
    let mut count: i32 = 0;
    let mut count_part2: i32 = 0;
    let mut locations: Vec<Pos> = Vec::new();
    let mut locations_part2: Vec<Pos> = Vec::new();
    for char in char_single {
        let same_chars: Vec<&OneChar> = all_chars.iter().filter(|c| c.char == char).collect();
        for first_char in &same_chars {
            for second_char in &same_chars {
                if first_char != second_char {
                    let distance = check_distance(first_char, second_char);
                    let mut location = calculate_location(second_char, distance);

                    if is_in_bounds(&location) && !locations.contains(&location) {
                        if !locations_part2.contains(&location) {
                            count_part2 += 1;
                        }
                        count += 1;
                        locations.push(location);
                    }
                    process_new_locations(&distance, &mut location, &mut locations, &mut locations_part2, &mut count_part2);
                }
                process_second_char(&second_char, &mut locations, &mut locations_part2, &mut count_part2);
            }
        }
    }
    println!("Part1: {count}");
    println!("Part2: {count_part2}");
}

fn calculate_location(second_char: &OneChar, distance: (i32, i32)) -> Pos {
    Pos(
        second_char.pos.0 + distance.0,
        second_char.pos.1 + distance.1,
    )
}

fn is_in_bounds(pos: &Pos) -> bool {
    (0..50).contains(&pos.0) && (0..50).contains(&pos.1)
}

fn process_new_locations(
    distance: &(i32, i32),
    location: &mut Pos,
    locations: &mut Vec<Pos>,
    locations_part2: &mut Vec<Pos>,
    count_part2: &mut i32,
) {
    while is_in_bounds(location) {
        location.0 += distance.0;
        location.1 += distance.1;

        if is_in_bounds(location)
            && !locations.contains(location)
            && !locations_part2.contains(location)
        {
            *count_part2 += 1;
            locations_part2.push(*location);
        }
    }
}

fn process_second_char(
    second_char: &OneChar,
    locations: &mut Vec<Pos>,
    locations_part2: &mut Vec<Pos>,
    count_part2: &mut i32,
) {
    if !locations_part2.contains(&second_char.pos) && !locations.contains(&second_char.pos) {
        *count_part2 += 1;
        locations_part2.push(second_char.pos);
    }
}

fn check_distance(first: &OneChar, next: &OneChar) -> (i32, i32) {
    (next.pos.0 - first.pos.0, next.pos.1 - first.pos.1)
}
