use std::fs::File;
use std::io::{self, Read};
use regex::Regex;


fn main() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Failed to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    let mut sum = 0;

    for caps in re.captures_iter(&content) {
        let num1: i32 = caps[1].parse().unwrap();
        let num2: i32 = caps[2].parse().unwrap();

        let product = num1 * num2;
        sum += product;
    }

    println!("Gesamtsumme der Produkte: {}", sum);
    Ok(())
}
