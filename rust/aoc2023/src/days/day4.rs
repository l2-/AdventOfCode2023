use std::cmp::*;
use std::collections::HashSet;

use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = 4;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));
    let mut sum = 0;
    let mut nr_of_cards  = vec![1i64; lines.len()];
    for i in 0..lines.len() {
        let line = &string_split(&lines[i], ":")[1];
        let split = string_split(line, "|");
        let left: HashSet<i64> = HashSet::from_iter(string_to_ints(&split[0]).into_iter());
        let right: HashSet<i64> = HashSet::from_iter(string_to_ints(&split[1]).into_iter());
        let intersection = left.intersection(&right).count();
        if intersection > 0 {
            sum += 2i64.pow((intersection - 1) as u32);
            for k in i+1..min(nr_of_cards.len(), i + intersection + 1) {
                nr_of_cards[k] += nr_of_cards[i];
            }
        }
    }
    println!("Part 1 {}", sum);
    println!("Part 2 {}", nr_of_cards.iter().sum::<i64>());
}
