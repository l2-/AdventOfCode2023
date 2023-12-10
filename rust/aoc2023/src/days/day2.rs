use super::super::ulitity::*;
use super::super::dayx::*;

pub fn day() {
    let redMax = 12; let greenMax = 13; let blueMax = 14;
    let day = 2;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));
    let mut sum = 0;
    let mut sump2 = 0;
    for line in lines.iter() {
        let line = line.replace(";", ",");
        let split1 = string_split(&line, ":");
        let split2 = string_split(&split1[1], ",");
        let gameid = string_to_ints(&line)[0];
        let mut result = true;
        let mut minRed = 0;
        let mut minGreen = 0;
        let mut minBlue = 0;
        for split in split2 {
            let split3 = string_split(&split.trim(), " ");
            result &= match split3[1].as_str() {
                "red" => string_to_ints(&split3[0])[0] <= redMax,
                "green" => string_to_ints(&split3[0])[0] <= greenMax,
                "blue" => string_to_ints(&split3[0])[0] <= blueMax,
                _ => true,
            };
            match split3[1].as_str() {
                "red" => minRed = std::cmp::max(minRed, string_to_ints(&split3[0])[0]),
                "green" => minGreen = std::cmp::max(minGreen, string_to_ints(&split3[0])[0]),
                "blue" => minBlue = std::cmp::max(minBlue, string_to_ints(&split3[0])[0]),
                _ => (),
            }
        }
        if result {
            sum += gameid;
        }
        sump2 += minRed * minBlue * minGreen;
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sump2);
}