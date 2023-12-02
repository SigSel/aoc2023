mod file_reader;
mod day_interface;
mod day1;
mod day2;

use day2::DayTwo;
use day_interface::DayInterface;

fn main() {
    let file_path = "input/day2.txt";
    match file_reader::read_file_lines(file_path){
        Ok(lines) => {
            let day = DayTwo;
            day.solve_puzzle_one(&lines);
            day.solve_puzzle_two(&lines);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
