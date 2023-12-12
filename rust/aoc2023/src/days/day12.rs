use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;
use std::time::Instant;

type State = (usize, usize);

//if final state and valid return 1
//for each group
//  remove group from string
//  pass remaining string and group

fn options(
    s: &String,
    sstart: usize,
    groups: &Vec<usize>,
    gstart: usize,
    cache: &mut HashMap<State, i64>,
) -> i64 {
    let state = (sstart, gstart);
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }

    // no more remaining string left
    if s.len() <= sstart {
        // valid if no more groups left either
        return if groups.len() <= gstart { 1 } else { 0 };
    }
    // no more groups left
    if groups.len() <= gstart {
        // valid if no more '#' left in the remaining string
        return if s.chars().skip(sstart).any(|e| e == '#') { 0 } else { 1 };
    }
    // for n groups we need at least n * 2 - 1 chars left since every group must have a '.' or '?' seperator
    if (groups.len() - gstart) * 2 - 1 > s.len() - sstart { return 0; }

    let mut ans = 0;
    let group = groups[gstart];
    let c = s.chars().nth(sstart).unwrap();
    if c == '#' || c == '?' {
        // current group must fit in the string we have left
        // no '.' in s covered by the group (lookahead)
        // the char in s after the group must be '.' (or '?')

        let cond1 =
            sstart + group <= s.len() && !(sstart..sstart + group).any(|i| s.chars().nth(i).unwrap() == '.');
        let cond2 = sstart + group >= s.len() || s.chars().nth(sstart + group).unwrap() != '#';
        if cond1 && cond2 {
            let _sstart: usize = sstart + group + 1;
            let _gstart: usize = gstart + 1;

            ans += options(&s, _sstart, &groups, _gstart, cache);
        }
    }
    if c == '.' || c == '?' {
        let _sstart: usize = sstart + 1;
        let _gstart: usize = gstart;

        ans += options(&s, _sstart, &groups, _gstart, cache);
    }

    cache.insert(state, ans);
    return ans;
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines = read_lines(&input_path(day));

    {
        let now = Instant::now();
        let mut col = vec![];
        for line in lines.iter() {
            let split = string_split(&line, " ");
            let groups: Vec<usize> = string_to_ints(&line).iter().map(|e| *e as usize).collect();
            col.push((split[0].clone(), groups));
        }
        println!("Lines in {:?}", now.elapsed());

        let parts = vec![(1, 1), (2, 5)];
        for &(part, repeating) in parts.iter() {
            let now = Instant::now();
            let mut ans = 0i64;
            for (s, group) in col.iter() {
                let s = (0..repeating)
                    .map(|_| s.to_string())
                    .collect::<Vec<String>>()
                    .join("?");
                let group = (0..repeating).flat_map(|_| group.iter().copied()).collect();
                let _ans = options(&s, 0, &group, 0, &mut HashMap::new());
                ans += _ans;
            }
            println!("Part {:?} {:?} in {:?}", part, ans, now.elapsed());
        }
    }
}
