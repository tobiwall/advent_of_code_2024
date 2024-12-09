use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct MatchData {
    start: usize,
    text: String,
    tipe: String,
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("Failed to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let re_mul = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut sum = 0;
    let mut ignore = false;
    let mut matches: Vec<MatchData> = Vec::new();

    for dont_caps in re_dont.captures_iter(&content) {
        let dont_match = dont_caps.get(0).unwrap();
        matches.push(MatchData {
            start: dont_match.start(),
            text: dont_match.as_str().to_string(),
            tipe: "dont".to_string(),
        });
    }

    for do_caps in re_do.captures_iter(&content) {
        let do_match = do_caps.get(0).unwrap();
        matches.push(MatchData {
            start: do_match.start(),
            text: do_match.as_str().to_string(),
            tipe: "do".to_string(),
        });
    }

    for caps in re_mul.captures_iter(&content) {
        let mul_match = caps.get(0).unwrap();
        matches.push(MatchData {
            start: mul_match.start(),
            text: mul_match.as_str().to_string(),
            tipe: "mul".to_string(),
        });
    }

    matches.sort_by_key(|m| m.start);

    for m in matches {
        if m.tipe == "dont" {
            ignore = true;
        }

        if m.tipe == "do" {
            ignore = false;
        }

        if m.tipe == "mul" && ignore == false {
            if let Some(caps) = re_mul.captures(&m.text) {
                let num1: i32 = caps[1].parse().unwrap();
                let num2: i32 = caps[2].parse().unwrap();
                let product = num1 * num2;
                sum += product;
            }
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}
