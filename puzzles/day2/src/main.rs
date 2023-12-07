use std::{fs, path::Path};

fn match_3_char(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "six" => "6",
        _ => input,
    }
}

fn match_4_char(input: &str) -> &str {
    match input {
        "four" => "4",
        "five" => "5",
        "nine" => "9",
        _ => input,
    }
}

fn match_5_char(input: &str) -> &str {
    match input {
        "three" => "3",
        "seven" => "7",
        "eight" => "8",
        _ => input,
    }
}

fn parse_to_number(input: &str) -> &str {
    match input.len() {
        3 => match_3_char(input),
        4 => match_4_char(input),
        5 => match_5_char(input),
        _ => input,
    }
}

fn calibrate(input: &str) -> i32 {
    // iter is one-time use, so we need to collect it to use it again
    let chars: Vec<char> = input.chars().collect();

    let first = chars.iter().find(|&b| b.is_ascii_digit()).clone();
    if first.is_none() {
        return 0;
    }

    let second = chars.iter().rev().find(|&b| b.is_ascii_digit());

    match (first, second) {
        (Some(first), Some(second)) => format!("{}{}", first, second).parse::<i32>().unwrap(),
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string(Path::new("inputs/day1.txt")).unwrap();

    let mut result = 0;

    input.lines().for_each(|line| {
        result += calibrate(line);
    });

    println!("{}", result);
}
