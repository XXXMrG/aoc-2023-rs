use std::{cmp, fs, path::Path, time::Instant};

fn main() {
    let input = fs::read_to_string(Path::new("inputs/day3.txt")).unwrap();

    let result = 0;

    let mut start_time = Instant::now();

    let mut schematic: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|line| {
        let item: Vec<char> = line.chars().collect();
        schematic.push(item);
    });
    println!("{:?}", schematic);
    let mut end_time = Instant::now();

    println!("{}, costed: {:?}", result, end_time - start_time);
}
