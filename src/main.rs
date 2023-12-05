
pub mod days;
pub mod utils;


use crate::utils::file_reader;
use crate::days::day_fetch::fetch_day;

fn main() {
    let day_to_run = "4";
    let file_path = format!("input/day{}.txt", day_to_run);
    match fetch_day(day_to_run){
        Ok(day) => {
            match file_reader::read_file_lines(&file_path){
                Ok(lines) => {
                    day.solve_puzzle_one(&lines);
                    day.solve_puzzle_two(&lines);
                }
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        Err(error) => print!("Error {}", error)
    }

}
