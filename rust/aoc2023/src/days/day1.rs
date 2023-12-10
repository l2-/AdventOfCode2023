use super::super::ulitity::*;
use super::super::dayx::*;

static DIGIT_MAP: &[(&str, &str)] = &[
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub fn day() {
    let day = 1;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));
    let mut sum = 0;
    for line in lines.iter() {
        let mut digits:String = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                digits += &c.to_string();
            }
        }
        sum += digits.chars().next().unwrap().to_digit(10).unwrap() * 10;
        sum += digits.chars().last().unwrap().to_digit(10).unwrap();
    }
    println!("Part 1 {}", sum);

    let mut sum = 0;
    for line in lines.iter() {
        let mut line = line.clone();
        for &(k, v) in DIGIT_MAP.iter() {
            line = line.replace(k, &(k.to_owned() + v + k));
        }
        let mut digits:String = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                digits += &c.to_string();
            }
        }
        sum += digits.chars().next().unwrap().to_digit(10).unwrap() * 10;
        sum += digits.chars().last().unwrap().to_digit(10).unwrap();
    }
    println!("Part 2 {}", sum);
}