use super::super::dayx::*;
use super::super::ulitity::*;

fn count_occurance(i: usize, s: &str, use_joker: bool) -> i32 {
    let c = s.chars().nth(i).unwrap();
    let mut count = 1;
    for k in 0..s.len() {
        if k == i { continue; }
        let c1 = s.chars().nth(k).unwrap();
        if c == c1 || (c1 == 'J' && use_joker) { count += 1; }
    }
    return count;
}

fn score(s: &str, use_joker: bool) -> i32 {
    let count = |s: &str, use_joker: bool| {
        let mut counts = (0..s.len()).map(|i| (i, count_occurance(i, s, use_joker))).collect::<Vec<(usize, i32)>>();
        counts.sort_by_key(|(_, c)| *c);
        return counts;
    };
    let forms2 = |s: &str, c: char, use_joker: bool| s.replace(c, "").replace(if use_joker {"J"} else {""}, "");
    match count(s, use_joker).last().unwrap() {
        (_, 5) => return 7,
        (_, 4) => return 6,
        (i, 3) => match count(&forms2(s, s.chars().nth(*i).unwrap(), use_joker), false).last().unwrap() {
            (_, 2) => return 5,
            (_, _) => return 4,
        },
        (i, 2) => match count(&forms2(s, s.chars().nth(*i).unwrap(), use_joker), false).last().unwrap() {
            // (_, 3) => return 5, impossible
            (_, 2) => return 3,
            (_, _) => return 2,
        },
        (_, _) => return 1,
    }
}

fn cmp(score1: i32, s1: &String, score2: i32, s2: &String, order: &Vec<char>) -> std::cmp::Ordering {
    if score1 > score2 {
        return std::cmp::Ordering::Greater;
    }
    if score1 < score2 {
        return std::cmp::Ordering::Less;
    }
    // break tie
    for i in 0..s1.len() {
        let c1 = s1.chars().nth(i).unwrap();
        let c2 = s2.chars().nth(i).unwrap();
        let p1 = order.iter().position(|&r| r == c1).unwrap();
        let p2 = order.iter().position(|&r| r == c2).unwrap();
        if p1 < p2 { return std::cmp::Ordering::Greater ; }
        if p1 > p2 { return std::cmp::Ordering::Less ; }
    }
    return std::cmp::Ordering::Equal;
}

pub fn day() {
    let day = 7;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let parts = vec![
        (1, vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'], false),
        (2, vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'], true)
    ];
    for (part, order, use_joker) in parts.iter() {
        let mut scores: Vec<(_, _, _)> = lines.iter().map(|l| {
            let split = string_split(l, " ");
            let l1 = split[0].clone();
            let l2 = split[1].clone();
            return (score(&l1, *use_joker), l1, l2.as_str().parse::<i64>().unwrap());
        }).collect();
        scores.sort_by(|(c1, s1, _), (c2, s2, _)| cmp(*c1, s1, *c2, s2, &order));
        // println!("scores {:?}", scores);
        let mut sum = 0;
        for i in 0..scores.len() {
            let (_, _, points) = scores[i];
            sum += (i as i64 + 1i64) * points;
        }
        println!("Part {:?} {:?}", part, sum);
    }
    // 246163188
    // 245794069
}