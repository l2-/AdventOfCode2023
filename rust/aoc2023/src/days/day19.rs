use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::time::Instant;

use super::super::dayx::*;
use super::super::ulitity::*;

type Cond = (usize, i64, std::cmp::Ordering, String);
type Entry = [i64; 4];

fn construct_ruleset_and_entries(lines: &Vec<String>) -> (HashMap<String, Vec<Cond>>, Vec<Entry>) {
    let mut ruleset = HashMap::new() as HashMap<String, Vec<Cond>>;
    let mut entries: Vec<Entry> = vec![];
    let mut iter = lines.iter();
    let mapping = HashMap::from([('x', 0usize), ('m', 1), ('a', 2), ('s', 3)]);
    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let split = string_split(&line, "{");
        let name = split[0].clone();
        let s_rule = split[1].clone().replace("}", "");
        let conditions = s_rule
            .split(",")
            .map(|s| {
                if s.contains(":") {
                    let split = string_split(s, ":");
                    return (
                        *mapping.get(&split[0].chars().next().unwrap()).unwrap(),
                        string_to_ints(&split[0])[0],
                        if split[0].contains("<") {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        },
                        split[1].clone(),
                    );
                }
                return (4usize, 0, std::cmp::Ordering::Equal, s.to_owned());
            })
            .collect();
        ruleset.insert(name, conditions);
    }
    for line in iter.by_ref() {
        let splits: Vec<i64> = string_split(&line.replace("{", "").replace("}", ""), ",")
            .iter()
            .map(|l| i64::from_str_radix(&string_split(&l, "=")[1], 10).unwrap())
            .collect();
        entries.push([splits[0], splits[1], splits[2], splits[3]]);
    }
    return (ruleset, entries);
}

fn part1(ruleset: &HashMap<String, Vec<Cond>>, entries: &Vec<Entry>) -> i64 {
    let entries: Vec<(Entry, bool)> = entries
        .iter()
        .map(|entry| {
            let mut current = "in";
            while current != "A" && current != "R" {
                let rules = ruleset.get(current).unwrap();
                for (i, n, ord, next) in rules.iter() {
                    if *ord == std::cmp::Ordering::Equal || entry[*i].cmp(n) == *ord {
                        current = next;
                        break;
                    }
                }
            }
            return (*entry, current == "A");
        })
        .collect();
    return entries.iter().fold(
        0,
        |acc, (n, p)| if *p { acc + n.iter().sum::<i64>() } else { acc },
    );
}

fn _part2(xmas: [Int2; 4], ruleset: &HashMap<String, Vec<Cond>>, key: &str) -> i64 {
    if key == "A" {
        return xmas.iter().fold(1, |acc, e| (e.1 - e.0 + 1) * acc);
    } else if key == "R" {
        return 0;
    }
    let mut xmas = xmas;
    let rules = ruleset.get(key).unwrap();
    let mut sum = 0;
    let mut rules_iter = rules.iter();
    for (i, n, ord, next) in rules_iter.by_ref() {
        // unconditional advance
        if *i >= xmas.len() {
            sum += _part2(xmas, ruleset, &next);
            continue;
        }
        let (low, high) = xmas[*i];
        let rt = if *ord == std::cmp::Ordering::Less {
            (low, min(*n - 1, high))
        } else {
            (max(low, *n + 1), high)
        };
        let rf = if *ord == std::cmp::Ordering::Less {
            (max(low, *n), high)
        } else {
            (low, min(*n, high))
        };
        if rt.0 <= rt.1 {
            sum += _part2(
                (0..xmas.len())
                    .map(|k: usize| if k == *i { rt } else { xmas[k] })
                    .collect::<Vec<Int2>>()
                    .try_into()
                    .unwrap(),
                ruleset,
                &next,
            );
        }
        if rf.1 < rf.0 {
            break;
        }
        xmas[*i] = rf;
    }
    return sum;
}

fn part2(ruleset: &HashMap<String, Vec<Cond>>) -> i64 {
    _part2([(1, 4000), (1, 4000), (1, 4000), (1, 4000)], ruleset, "in")
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    {
        let now = Instant::now();
        let (ruleset, entries) = construct_ruleset_and_entries(&lines);
        println!("Parsing {:?}s", now.elapsed().as_secs_f64());
        let now = Instant::now();
        println!("Part 1 {:?} {:?}s", part1(&ruleset, &entries), now.elapsed().as_secs_f64());
        let now = Instant::now();
        println!("Part 2 {:?} {:?}s", part2(&ruleset), now.elapsed().as_secs_f64());
    }
}
