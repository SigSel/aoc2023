
use crate::day_interface::DayInterface;

pub struct DayOne;

impl DayInterface for DayOne{
    fn solve_puzzle_one(&self, lines: &Vec<String>) {
        let mut total_sum = 0;
        for line in lines {
            let first_number = line.chars()
                .find(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .unwrap_or(0) as i32;
            let last_number = line.chars()
                .rev()
                .find(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .unwrap_or(0) as i32;
            let sum = first_number * 10 + last_number;
            total_sum += sum;
        }
        println!("Total sum : {}", total_sum)

    }
    fn solve_puzzle_two(&self, lines: &Vec<String>) {
        for line in lines {
            println!("{}", line)
        }
    }
}