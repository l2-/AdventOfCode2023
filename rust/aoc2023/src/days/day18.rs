use super::super::dayx::*;
use super::super::ulitity::*;

fn steps_to_lines(steps: &Vec<((i64, i64), i64)>) -> Vec<Int2> {
    let mut current = (0i64, 0i64) as Int2;
    let mut points = vec![] as Vec<(i64, i64)>;
    points.push(current);
    for &(dir, s) in steps.iter() {
        current = current.add(dir.mul(s));
        points.push(current);
    }
    return points;
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let parts = vec![
        (
            1i64,
            (|line| {
                let split = string_split(line, " ");
                if split.len() != 3 {
                    return ((0i64, 0i64), 0i64);
                }
                let ints = string_to_ints(&split[1]);
                if ints.len() == 0 {
                    return ((0i64, 0i64), 0i64);
                }
                let nr_of_steps = string_to_ints(&split[1])[0];
                let dir = match split[0].chars().nth(0).unwrap() {
                    'U' => (0i64, -1i64),
                    'D' => (0i64, 1i64),
                    'L' => (-1i64, 0i64),
                    'R' => (1i64, 0i64),
                    _ => (0i64, 0i64),
                };
                return (dir, nr_of_steps);
            }) as fn(&String) -> (Int2, i64),
        ),
        (
            2i64,
            (|line| {
                let split = string_split(line, " ");
                if split.len() != 3 {
                    return ((0i64, 0i64), 0i64);
                }
                let hex = split[2].clone().replace("(", "").replace(")", "").replace("#", "");
                let nr_of_steps = hex_to_int(&hex.substring(0, hex.len() - 1));
                let dir = match hex_to_int(&hex.substring(hex.len() - 1, hex.len())) {
                    3 => (0i64, -1i64),
                    1 => (0i64, 1i64),
                    2 => (-1i64, 0i64),
                    0 => (1i64, 0i64),
                    _ => (0i64, 0i64),
                };
                return (dir, nr_of_steps);
            }) as fn(&String) -> (Int2, i64),
        ),
    ];
    for &(part, f) in parts.iter() {
        let steps: Vec<((i64, i64), i64)> = lines.iter().map(|l| f(l)).collect();
        let nr_of_boundaries = steps.iter().fold(0, |acc, &(_, s)| acc + s);
        let points = steps_to_lines(&steps);
        let inner_area = trapezoid_formula(&mut points.iter()) - nr_of_boundaries / 2 + 1;
        let area = inner_area + nr_of_boundaries;
        println!("Part {:?} {:?}", part, area);
    }
}
