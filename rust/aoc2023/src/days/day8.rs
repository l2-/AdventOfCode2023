use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = 8;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    {
        let lrs: Vec<char> = lines[0].chars().collect();
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        for i in 2..lines.len() {
            let a = lines[i].to_string().substring(0, 3);
            let b = lines[i].to_string().substring(7, 3);
            let c = lines[i].to_string().substring(12, 3);
            map.insert(a, (b, c));
        }

        let part1: (i32, Vec<String>, fn(&String) -> bool) =
            (1, vec!["AAA".to_string()], |s| s == "ZZZ");
        let part2: (i32, Vec<String>, fn(&String) -> bool) = (
            2,
            map.keys()
                .filter(|k| k.chars().nth(2).unwrap() == 'A')
                .map(|s| s.to_string())
                .collect(),
            |s| s.chars().nth(2).unwrap() == 'Z',
        );
        let parts = vec![part1, part2];
        for (part, starts, endfn) in parts.iter() {
            let min_steps: Vec<u64> = starts
                .iter()
                .map(|start| {
                    let mut steps = 0;
                    let mut c = start.to_string();
                    while !endfn(&c) {
                        let lr = lrs[steps % lrs.len()];
                        let next1 = map.get(&c).unwrap();
                        let next2 = match lr {
                            'R' => next1.1.clone(),
                            _ => next1.0.clone(),
                        };
                        c = next2;
                        steps += 1;
                    }
                    return steps as u64;
                })
                .collect();
            
            println!("Part {:?} {:?}", part, lcm(&min_steps));
        }
    }
}
