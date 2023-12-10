use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));
}