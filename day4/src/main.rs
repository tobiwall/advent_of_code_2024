// use std::{
//     fs::File,
//     io::{BufRead, BufReader},
// };

// fn main() {
//     let path = "input.txt";
//     let file = File::open(path).expect("Failed to open the file");
//     let reader = BufReader::new(file);
//     let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

//     let result = count_xmas(&lines);
//     println!("This is the result: {result}");
// }

// fn count_xmas(lines: &[String]) -> i32 {
//     let mut total_count = 0;
//     let directions = [
//         (1, 0),
//         (-1, 0),
//         (0, 1),
//         (0, -1),
//         (1, 1),
//         (1, -1),
//         (-1, 1),
//         (-1, -1),
//     ];

//     for (line_num, line) in lines.iter().enumerate() {
//         for (char_index, _) in line.chars().enumerate() {
//             for (dx, dy) in directions.iter() {
//                 if check_direction(line_num as i32, char_index as i32, *dx, *dy, lines) {
//                     total_count += 1;
//                 }
//             }
//         }
//     }

//     total_count
// }

// fn check_direction(
//     start_row: i32,
//     start_col: i32,
//     dx: i32,
//     dy: i32,
//     lines: &[String],
// ) -> bool {
//     let word = ['X', 'M', 'A', 'S'];
//     for (i, &letter) in word.iter().enumerate() {
//         let row = start_row + i as i32 * dx;
//         let col = start_col + i as i32 * dy;
//         if row < 0
//             || col < 0
//             || row as usize >= lines.len()
//             || col as usize >= lines[row as usize].len()
//             || lines[row as usize].chars().nth(col as usize) != Some(letter)
//         {
//             return false;
//         }
//     }
//     true
// }

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let result = count_xmas(&lines);
    println!("This is the result: {result}");
}

fn count_xmas(lines: &[String]) -> i32 {
    let mut total_count = 0;

    for (line_num, line) in lines.iter().enumerate() {
        for (char_index, _) in line.chars().enumerate() {
            if line.chars().nth(char_index) == Some('A') {
                if check_xmas(line_num, char_index, lines) {
                    total_count += 1;
                }
            }
        }
    }

    total_count
}

fn check_xmas(line_num: usize, char_index: usize, lines: &[String]) -> bool {
    let mut valid = false;

    if line_num > 0
        && char_index > 0
        && line_num + 1 < lines.len()
        && char_index + 1 < lines[line_num + 1].len()
        && lines[line_num - 1].chars().nth(char_index - 1) == Some('M')
        && lines[line_num + 1].chars().nth(char_index - 1) == Some('M')
        && lines[line_num - 1].chars().nth(char_index + 1) == Some('S')
        && lines[line_num + 1].chars().nth(char_index + 1) == Some('S')
    {
        valid = true;
    }

    if line_num > 0
        && char_index > 0
        && line_num + 1 < lines.len()
        && char_index + 1 < lines[line_num + 1].len()
        && lines[line_num - 1].chars().nth(char_index - 1) == Some('S')
        && lines[line_num + 1].chars().nth(char_index - 1) == Some('S')
        && lines[line_num - 1].chars().nth(char_index + 1) == Some('M')
        && lines[line_num + 1].chars().nth(char_index + 1) == Some('M')
    {
        valid = true;
    }

    if line_num + 1 < lines.len()
        && char_index > 0
        && char_index + 1 < lines[line_num + 1].len()
        && lines[line_num + 1].chars().nth(char_index - 1) == Some('S')
        && lines[line_num + 1].chars().nth(char_index + 1) == Some('S')
        && line_num > 0
        && lines[line_num - 1].chars().nth(char_index - 1) == Some('M')
        && lines[line_num - 1].chars().nth(char_index + 1) == Some('M')
    {
        valid = true;
    }

    if line_num + 1 < lines.len()
        && char_index > 0
        && char_index + 1 < lines[line_num + 1].len()
        && lines[line_num + 1].chars().nth(char_index - 1) == Some('M')
        && lines[line_num + 1].chars().nth(char_index + 1) == Some('M')
        && line_num > 0
        && lines[line_num - 1].chars().nth(char_index - 1) == Some('S')
        && lines[line_num - 1].chars().nth(char_index + 1) == Some('S')
    {
        valid = true;
    }

    valid
}

/*
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "input.txt";
    let file = File::open(path).expect("Failed to open the file");
    let reader = BufReader::new(file);
    let mut sum_result = 0;

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for (line_num, line) in lines.iter().enumerate() {
        let (sum_forward, x_positions) = search_forward(line);
        sum_result += sum_forward;

        sum_result += search_down(&x_positions, &lines, line_num as i32);
        sum_result += search_diagonal(&x_positions, &lines, line_num as i32);

        let (sum_backwards, s_positions) = search_backwards(line);
        sum_result += sum_backwards;

        sum_result += search_up(&s_positions, &lines, line_num as i32);
        sum_result += search_diagonal_up(&s_positions, &lines, line_num as i32);
    }

    println!("This is the result: {sum_result}");
}

fn search_forward(line: &str) -> (i32, Vec<usize>) {
    let mut x_position: Vec<usize> = Vec::new();
    let mut sum = 0;
    let mut start = 0;
    while let Some(position) = line[start..].find('X') {
        let absolute_position = start + position;
        x_position.push(absolute_position);
        if absolute_position + 3 < line.len() {
            let substring = &line[absolute_position + 1..absolute_position + 4];
            if substring == "MAS" {
                sum += 1;
            }
        }
        start = absolute_position + 1;
    }
    (sum, x_position)
}

fn search_backwards(line: &str) -> (i32, Vec<usize>) {
    let mut s_position: Vec<usize> = Vec::new();
    let mut sum = 0;
    let mut start = 0;
    while let Some(position) = line[start..].find('S') {
        let absolute_position = start + position;
        s_position.push(absolute_position);
        if absolute_position + 3 < line.len() {
            let substring = &line[absolute_position + 1..absolute_position + 4];
            if substring == "AMX" {
                sum += 1;
            }
        }
        start = absolute_position + 1;
    }
    (sum, s_position)
}

fn search_down(x_positions: &Vec<usize>, lines: &Vec<String>, line_num: i32) -> i32 {
    let mut sum = 0;

    for x in x_positions {
        if line_num < 137 {
            if let (Some(next_line1), Some(next_line2), Some(next_line3)) = (
                lines.get((line_num + 1) as usize),
                lines.get((line_num + 2) as usize),
                lines.get((line_num + 3) as usize),
            ) {
                if next_line1.chars().nth(*x) == Some('M')
                    && next_line2.chars().nth(*x) == Some('A')
                    && next_line3.chars().nth(*x) == Some('S')
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn search_up(s_positions: &Vec<usize>, lines: &Vec<String>, line_num: i32) -> i32 {
    let mut sum = 0;

    for s in s_positions {
        if line_num < 137 {
            if let (Some(next_line1), Some(next_line2), Some(next_line3)) = (
                lines.get((line_num + 1) as usize),
                lines.get((line_num + 2) as usize),
                lines.get((line_num + 3) as usize),
            ) {
                if next_line1.chars().nth(*s) == Some('A')
                    && next_line2.chars().nth(*s) == Some('M')
                    && next_line3.chars().nth(*s) == Some('X')
                {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn search_diagonal(x_positions: &Vec<usize>, lines: &Vec<String>, line_num: i32) -> i32 {
    let mut sum = 0;

    if line_num < 137 {
        if let (Some(next_line1), Some(next_line2), Some(next_line3)) = (
            lines.get((line_num + 1) as usize),
            lines.get((line_num + 2) as usize),
            lines.get((line_num + 3) as usize),
        ) {
            for x in x_positions {
                if *x < 137
                    && next_line1.chars().nth(x + 1) == Some('M')
                    && next_line2.chars().nth(x + 2) == Some('A')
                    && next_line3.chars().nth(x + 3) == Some('S')
                {
                    sum += 1;
                }

                if *x > 3
                    && next_line1.chars().nth(x - 1) == Some('M')
                    && next_line2.chars().nth(x - 2) == Some('A')
                    && next_line3.chars().nth(x - 3) == Some('S')
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn search_diagonal_up(s_positions: &Vec<usize>, lines: &Vec<String>, line_num: i32) -> i32 {
    let mut sum = 0;

    if line_num < 137 {
        if let (Some(next_line1), Some(next_line2), Some(next_line3)) = (
            lines.get((line_num + 1) as usize),
            lines.get((line_num + 2) as usize),
            lines.get((line_num + 3) as usize),
        ) {
            for s in s_positions {
                if *s < 137
                    && next_line1.chars().nth(s + 1) == Some('A')
                    && next_line2.chars().nth(s + 2) == Some('M')
                    && next_line3.chars().nth(s + 3) == Some('X')
                {
                    sum += 1;
                }
                if *s > 3
                    && next_line1.chars().nth(s - 1) == Some('A')
                    && next_line2.chars().nth(s - 2) == Some('M')
                    && next_line3.chars().nth(s - 3) == Some('X')
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}
 */
