mod file_reader;
mod day_interface;
mod day1;

use day1::DayOne;
use day_interface::DayInterface;

fn main() {
    let file_path = "input/day1.txt";
    match file_reader::read_file_lines(file_path){
        Ok(lines) => {
            let day_one = DayOne;
            day_one.solve_puzzle_one(&lines)
            // Do further processing with the array of strings
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
