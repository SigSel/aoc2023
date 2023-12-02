use crate::day_interface::DayInterface;

pub struct DayTwo;


impl DayInterface for DayTwo {
    fn solve_puzzle_one(&self, lines: &Vec<String>) {
        let (red_cubes, green_cubes, blue_cubes) = (12, 13, 14);
        let mut total_sum = 0;
        for line in lines {
            let (game, game_info) = line.split_once(':').unwrap_or(("", ""));
            let (_, game_id) = game.split_once(' ').unwrap_or(("", ""));
            if is_game_possible(game_info, red_cubes, blue_cubes, green_cubes) {
                match game_id.parse::<i32>() {
                    Ok(parsed_number) => {
                        total_sum += parsed_number;
                    }
                    Err(_) => {
                        println!("Failed to parse the number");
                    }
                }
            }
        }
        println!("Total sum of valid games: {}", total_sum)
    }
    fn solve_puzzle_two(&self, lines: &Vec<String>) {
        let mut total_sum = 0;
        for line in lines {
            let (_, game_info) = line.split_once(':').unwrap_or(("", ""));
            total_sum += product_of_minimal_number_of_cubes(game_info);
        }
        println!("Total sum of valid games: {}", total_sum)
    }
}

fn is_game_possible(
    game_info: &str, total_red_cubes: i32, total_blue_cubes: i32, total_green_cubes: i32
) -> bool {
    let game_steps: Vec<&str> = game_info.split(";").collect();
    let (mut red_cubes, mut blue_cubes, mut green_cubes) = (total_red_cubes, total_blue_cubes, total_green_cubes);
    for game_step in game_steps {
        let game_moves: Vec<&str> = game_step.split(",").collect();
        for game_move in game_moves {
            let (cubes, color) = split_game_move(game_move);

            if color == "red" {
                red_cubes -= cubes;
                if red_cubes < 0 {
                    return false
                }
            }
            else if color == "blue" {
                blue_cubes -= cubes;
                if blue_cubes < 0 {
                    return false
                }
            }
            else if color == "green" {
                green_cubes -= cubes;
                if green_cubes < 0 {
                    return false
                }
            }
        }
        red_cubes = total_red_cubes;
        blue_cubes = total_blue_cubes;
        green_cubes = total_green_cubes;
    }
    return true
}

fn product_of_minimal_number_of_cubes(game_info: &str) -> i32 {
    let game_steps: Vec<&str> = game_info.split(";").collect();
    let (mut max_red_cubes, mut max_blue_cubes, mut max_green_cubes) = (0, 0, 0);

    for game_step in game_steps {
        let game_moves: Vec<&str> = game_step.split(",").collect();
        for game_move in game_moves {
            let (cubes, color) = split_game_move(game_move);
            if color == "red" {
                if max_red_cubes < cubes {
                    max_red_cubes = cubes
                }
            }
            else if color == "blue" {
                if max_blue_cubes < cubes {
                    max_blue_cubes = cubes
                }
            }
            else if color == "green" {
                if max_green_cubes < cubes {
                    max_green_cubes = cubes
                }
            }
        }
    }

    return max_red_cubes * max_blue_cubes * max_green_cubes
}

fn split_game_move(game_move: &str) -> (i32, &str) {
    let mut cubes = 0;
    let mut color = "f";
    for info in game_move.split_whitespace(){
        match info.parse::<i32>() {
            Ok(parsed_number) => {
                cubes = parsed_number;
            }
            Err(_) => {
                color = info
            }
        }
    }
    return (cubes, color)
}
