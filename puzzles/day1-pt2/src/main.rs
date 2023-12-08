use regex::Regex;
use std::{fs, path::Path, time::Instant};

fn find_normal(input: &str) -> i32 {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let result = re.find(input).unwrap();

    match result.as_str() {
        number if number.chars().count() == 1 => number.parse::<i32>().unwrap(),
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn find_rev(input: &str) -> i32 {
    let re = Regex::new(r"(\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    let result = re.find(input).unwrap();

    match result.as_str() {
        number if number.chars().count() == 1 => number.parse::<i32>().unwrap(),
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => 0,
    }
}

fn calibrate(input: &str) -> i32 {
    let first = find_normal(input);
    let second = find_rev(&input.chars().rev().collect::<String>());

    first * 10 + second
}

fn main() {
    let input = fs::read_to_string(Path::new("inputs/day1.txt")).unwrap();

    let mut result = 0;

    let start_time = Instant::now();
    input.lines().for_each(|line| {
        let res = calibrate(&line);
        // println!("{} >> {}", line, res);
        result += res;
    });
    let end_time = Instant::now();

    println!("{}, costed: {:?}", result, end_time - start_time);
}
