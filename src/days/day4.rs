use crate::days::day_interface::DayInterface;


pub struct DayFour;


impl DayInterface for DayFour {
    fn solve_puzzle_one(&self, lines: &Vec<String>) {
        let mut total_sum = 0;
        for line in lines{
            let (list_of_winning_numbers, list_of_drawn_numbers) = extract_line_information(line);
            let common_elements = find_common_elements(list_of_winning_numbers, list_of_drawn_numbers);
            let base_number = 2;
            if common_elements.len() == 0 {
                continue
            }
            total_sum += power(base_number, common_elements.len() - 1);
        }
        println!("Total sum: {}", total_sum)

    }
    fn solve_puzzle_two(&self, lines: &Vec<String>) {
        println!("{}", lines[0])
    }
}

fn extract_line_information(line: &str) -> (Vec<&str>, Vec<&str>) {
    let (_, game_info) = line.split_once(':').unwrap_or(("", ""));
    let (winning_numbers, drawn_numbers) = game_info
        .split_once('|')
        .unwrap_or(("", ""));
    let list_of_winning_numbers: Vec<_> = winning_numbers.split_whitespace().collect();
    let list_of_drawn_numbers: Vec<_> = drawn_numbers.split_whitespace().collect();
    (list_of_winning_numbers, list_of_drawn_numbers)
}

fn find_common_elements<'a>(input_one: Vec<&'a str>, input_two: Vec<&'a str>) -> Vec<&'a str> {
    let common_elements: Vec<_> = input_one.iter()
        .filter(|&element1| input_two
            .iter().any(|&element2| element1 == &element2)
        ).cloned().collect();
    common_elements
}

fn power(base: i32, exponent: usize) -> i32 {
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
    }
    result
}
