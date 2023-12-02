
use crate::days::day_interface::DayInterface;
use regex::Regex;

pub struct DayOne;

fn find_first_and_last_number(line: &str) -> (i32, i32) {
    let first_number = line.chars()
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0) as i32;
    let last_number = line.chars()
        .rev()
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0) as i32;

    (first_number, last_number)
}

fn find_first_number_with_letters(line: &str) -> Option<i32> {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut first_match: Option<i32> = None;

    for i in 0..line.len() {
        if let Some(mat) = re.find_at(line, i) {
            first_match = get_digit_value(mat.as_str());
            break;
        }
    }
    return first_match
}

fn find_last_number_with_letters(line: &str) -> Option<i32> {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut last_match: Option<i32> = None;

    for i in 0..line.len() {
        if let Some(mat) = re.find_at(line, i) {
            last_match = get_digit_value(mat.as_str());
        }
    }
    return last_match
}

fn get_digit_value(digit_str: &str) -> Option<i32> {
    match digit_str {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
impl DayInterface for DayOne {
    fn solve_puzzle_one(&self, lines: &Vec<String>) {
        let mut total_sum = 0;
        for line in lines {
            let (first_number, last_number) = find_first_and_last_number(line);
            let sum = first_number * 10 + last_number;
            total_sum += sum;
        }
        println!("Total sum : {}", total_sum)
    }
    fn solve_puzzle_two(&self, lines: &Vec<String>) {
        let mut total_sum = 0;
        for line in lines {
            let (mut first_number, mut last_number) = find_first_and_last_number(line);
            let index_first_number = line.find(char::is_numeric).unwrap_or(0);
            let reversed_line: String = line.chars().rev().collect();
            let index_last_number = reversed_line.find(char::is_numeric).unwrap_or(0);

            let (part_before_first_digit, _) = line.split_at(index_first_number);
            match find_first_number_with_letters(part_before_first_digit){
                Some(value) => {
                    first_number = value;
                }
                None => {}
            }

            let (part_before_last_digit_reversed, _) = reversed_line.split_at(index_last_number);
            let part_before_last_digit: String = part_before_last_digit_reversed.chars().rev().collect();
            match find_last_number_with_letters(&part_before_last_digit){
                Some(value) => {
                    last_number = value
                }
                None => {}
            }
            let sum = first_number * 10 + last_number;
            total_sum += sum;
        }
        println!("Total_sum puzzle2 : {}", total_sum)
    }
}
