
use crate::days::day_interface::DayInterface;
use crate::days::{
    day1::DayOne,
    day2::DayTwo,
    day4::DayFour,
};

pub fn fetch_day(day: &str) -> Result<Box<dyn DayInterface>, &'static str> {
    match day {
        "1" => Ok(Box::new(DayOne)),
        "2" => Ok(Box::new(DayTwo)),
        "4" => Ok(Box::new(DayFour)),
        _ => Err("error"),
    }
}
