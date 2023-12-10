
use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = 6;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let l = lines[0].replace(" ", "");
    let timeslines = vec![&lines[0], &l];
    let l = lines[1].replace(" ", "");
    let distancelines = vec![&lines[1], &l];
    for k in 0..timeslines.len() {
        let mut sum = 1;
        let times = string_to_ints(&timeslines[k]);
        let distances = string_to_ints(&distancelines[k]);

        for i in 0..times.len() {
            // distance = (total time - time held) * time held = -x^2+bx-c > 0
            // a = -1, c = -distance, b = total_time.
            let total_time = times[i]; // b
            let distance = distances[i]; // c
            let first = total_time * total_time - 4 * distance; //b^2-4ac
            if first <= 0 {
                println!("Impossible for {} {} {} {}", total_time, distance, first, i);
                continue;
            }
            let second = ((-total_time as f64 + f64::sqrt(first as f64)) / -2f64) as i64;
            let third = ((-total_time as f64 - f64::sqrt(first as f64)) / -2f64 - 0.01f64) as i64;

            sum *= third - second;
            println!("ans {} {} {} {} ans {}", total_time, distance, second, third, third - second);
        }
        println!("Part {} {}", k + 1, sum);
    }
}