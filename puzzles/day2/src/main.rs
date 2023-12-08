use std::{fs, path::Path, time::Instant};

const MAX_RED: i32 = 12;
const MAX_BLUE: i32 = 14;
const MAX_GREEN: i32 = 13;

fn get_a_possible_game_id(input: &str) -> i32 {
    let input_vec = input.split(":").collect::<Vec<&str>>();
    let (game_info, game_data) = (input_vec[0], input_vec[1]);

    // input: Game 1: 4 red, get 1.
    let game_id = game_info.split(" ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let mut one_round_input_iter = game_data.split(";").into_iter();
    let impossible = one_round_input_iter.any(|one_round_input| {
        one_round_input.split(",").into_iter().any(|s| {
            let input_vec: Vec<&str> = s.split(" ").collect::<Vec<&str>>();
            let (amount, color) = (input_vec[1].parse::<i32>().unwrap(), input_vec[2]);

            match color {
                "red" => amount > MAX_RED,
                "blue" => amount > MAX_BLUE,
                "green" => amount > MAX_GREEN,
                _ => true,
            }
        })
    });

    if impossible {
        return 0;
    }

    game_id
}

fn main() {
    let input = fs::read_to_string(Path::new("inputs/day2.txt")).unwrap();

    let mut result = 0;

    let start_time = Instant::now();
    input.lines().for_each(|line| {
        result += get_a_possible_game_id(line);
    });
    let end_time = Instant::now();

    println!("{}, costed: {:?}", result, end_time - start_time);
}
