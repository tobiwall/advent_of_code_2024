use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input_vec = Columns {
        left_column: Vec::new(),
        right_column: Vec::new(),
    };

    input_vec.read_file();
    // input_vec.sort_column();
    // let sum_difference = input_vec.compare_numbers();

    // println!("The result is: {sum_difference}");



    let sum = input_vec.get_all_num();
    println!("The sum of all includes numbers is {sum}");
    
}

struct Columns {
    left_column: Vec<i32>,
    right_column: Vec<i32>,
}

impl Columns {
    fn read_file(&mut self) {
        let path = "input.txt";
        let file = File::open(path).expect("Could not open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 2 {
                if let Ok(left) = parts[0].parse::<i32>() {
                    self.left_column.push(left);
                }
                if let Ok(right) = parts[1].parse::<i32>() {
                    self.right_column.push(right);
                }
            }
        }
    }

    fn sort_column(&mut self) {
        self.left_column.sort();
        self.right_column.sort();
    }

    fn compare_numbers(&mut self) -> i32 {
        let mut i = 0;
        let mut sum_difference = 0;
        for line in self.left_column.clone() {
            let difference = difference(line, self.right_column[i]);
            i += 1;
            sum_difference += difference;
        }
        sum_difference
    }

    fn get_all_num(&mut self) -> i32 {
        let mut num_vec= 0;
        for &num in &self.left_column {
            let count: i32 = self.right_column.iter().filter(|&&x| x == num).count() as i32;
            num_vec += num * count;
        }
        num_vec
    }
}

fn difference(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

