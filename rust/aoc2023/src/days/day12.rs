use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;

type State = (usize, usize);

//if final state and valid return 1
//for each group
//  remove group from string
//  pass remaining string and group

fn options(s: &String, sstart: usize, groups: &Vec<usize>, gstart: usize, cache: &mut HashMap<State, i64>) -> i64 {
    if groups.len() <= gstart {
        if !s.chars().skip(sstart).any(|c| c == '#') {
            return 1;
        }
        return 0;
    } else if !s.chars().skip(sstart).any(|c| c == '#' || c == '?') {
        return 0;
    }

    let mut ans = 0;
    let group = groups[gstart];
    for i in sstart..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c == '#' || c == '?' {
            let cond1 =
                i + group <= s.len() && !(i..i + group).any(|i| s.chars().nth(i).unwrap() == '.');
            let cond2 = i + group >= s.len() || s.chars().nth(i + group).unwrap() != '#';
            if cond1 && cond2 {
                let _sstart: usize = i + group + 1;
                let _gstart: usize = gstart + 1;

                // let sl = s.substring(0, i);
                // let sr = s.substring(i + group, s.len() - i + group);
                // println!("{:?}", sl + &"x".repeat(group) + &sr);

                let state = (_sstart, _gstart);
                if !cache.contains_key(&state) {
                    let ans = options(&s, _sstart, &groups, _gstart, cache);
                    cache.insert(state, ans);
                }
                ans += cache.get(&state).unwrap();
            }
            if c == '#' {
                break;
            }
        }
    }

    return ans;
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines = read_lines(&input_path(day));

    {
        let mut col = vec![];
        for line in lines.iter() {
            let split = string_split(&line, " ");
            let groups: Vec<usize> = string_to_ints(&line).iter().map(|e| *e as usize).collect();
            col.push((split[0].clone(), groups));
        }

        let parts = vec![
            (1, 1),
            (2, 5)
        ];
        for &(part, repeating) in parts.iter() {
            use std::time::Instant;
            let now = Instant::now();
            let mut ans = 0i64;
            for (s, group) in col.iter() {
                let s = (0..repeating).map(|_| s.to_string()).collect::<Vec<String>>().join("?");
                let group = (0..repeating)
                    .flat_map(|_| group.iter().copied())
                    .collect();
                let _ans = options(&s, 0, &group, 0, &mut HashMap::new());
                ans += _ans;
            }
            println!("Part {:?} {:?} in {:?}", part, ans, now.elapsed());
        }
    }
}
