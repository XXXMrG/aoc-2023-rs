use std::{fs, path::Path, time::Instant};

fn is_symbol(symbol: &char) -> bool {
    symbol != &'.'
}

fn has_symbol_in_start_or_end(line: &Vec<char>, start: usize, end: usize) -> bool {
    if start > 0 {
        match line.get(start - 1) {
            Some(symbol) => {
                if is_symbol(symbol) {
                    return true;
                }
            }
            None => {}
        };
    }

    // right
    match line.get(end + 1) {
        Some(symbol) => {
            if is_symbol(symbol) {
                return true;
            }
        }
        None => {}
    }

    false
}

fn sun_part_number(input: &Schematic) -> i32 {
    let row_len = input[0].len();
    let col_len = input.len();

    let mut result = 0;

    let mut add_symbol = |position: (usize, usize), row_index: usize, part_number: i32| {
        let (start, end) = position;
        let row_line = &input[row_index];

        if has_symbol_in_start_or_end(row_line, start, end) {
            println!("{}: left or right: {}", row_index, part_number);
            result += part_number;
            return;
        }

        // up
        if row_index > 0 {
            let up_row = &input[row_index - 1];

            if has_symbol_in_start_or_end(up_row, start, end)
                || up_row[start..end + 1]
                    .iter()
                    .find(|item| is_symbol(item))
                    .is_some()
            {
                println!("{}: up: {}", row_index, part_number);
                result += part_number;
                return;
            }
        }

        // bottom
        if row_index < col_len - 1 {
            let bottom_row = &input[row_index + 1];

            if has_symbol_in_start_or_end(bottom_row, start, end)
                || bottom_row[start..end + 1]
                    .iter()
                    .find(|item| is_symbol(item))
                    .is_some()
            {
                println!("{}: bottom: {}", row_index, part_number);
                result += part_number;
                return;
            }
        }
    };

    input.iter().enumerate().for_each(|(row_index, row)| {
        let mut number_start = 0;
        let mut number_end = 0;
        let mut number_chars: Vec<char> = Vec::new();
        let mut has_number_flag = false;

        row.iter().enumerate().for_each(|(col_index, col)| {
            if has_number_flag {
                if col.is_ascii_alphanumeric() {
                    number_chars.push(*col);
                } else {
                    has_number_flag = false;
                    number_end = col_index - 1;

                    let part_number_str: String = number_chars.iter().collect();
                    let part_number: i32 = part_number_str.parse().unwrap();
                    add_symbol((number_start, number_end), row_index, part_number);

                    number_chars.clear();
                }
            } else {
                if col.is_ascii_alphanumeric() {
                    number_start = col_index;
                    number_chars.push(*col);
                    has_number_flag = true;
                }
            }

            if col_index == row_len - 1 && col.is_ascii_alphanumeric() {
                has_number_flag = false;
                number_end = col_index;

                let part_number_str: String = number_chars.iter().collect();
                let part_number: i32 = part_number_str.parse().unwrap();
                add_symbol((number_start, number_end), row_index, part_number);
                println!("{}", part_number);

                number_chars.clear();
            }
        });
    });

    result
}

type Schematic = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string(Path::new("inputs/day3.txt")).unwrap();

    let start_time = Instant::now();

    let mut schematic: Schematic = Vec::new();
    input.lines().for_each(|line| {
        let item: Vec<char> = line.chars().collect();
        schematic.push(item);
    });
    let result = sun_part_number(&schematic);
    let end_time = Instant::now();

    println!("{}, costed: {:?}", result, end_time - start_time);
}
