use super::super::dayx::*;
use super::super::ulitity::*;

fn parse(line: &String) -> Vec<(Vec<String>, Vec<String>)> {
    let cols = |lines: &Vec<String>| transpose_strings(&lines);
    let rows = |lines: &Vec<String>| lines.to_owned();

    return line
        .split("\r\n\r\n")
        .map(|line| line.lines().map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|lines| (cols(&lines), rows(&lines)))
        .collect();
}

fn compare_str(s1: &String, s2: &String) -> i32 {
    (0..std::cmp::min(s1.len(), s2.len()))
        .into_iter()
        .map(|i| {
            if s1.chars().nth(i).unwrap() == s2.chars().nth(i).unwrap() {
                0
            } else {
                1
            }
        })
        .sum()
}

// mismatches are smudges
fn find_mirror(strings: &Vec<String>, i: i32, k: i32, allowed_mismatches: i32) -> bool {
    if i < 0 || k as usize >= strings.len() {
        // we want to have used exactly the number of allowed mismatches
        return allowed_mismatches <= 0;
    }
    let mismatches = compare_str(&strings[i as usize], &strings[k as usize]);
    if compare_str(&strings[i as usize], &strings[k as usize]) <= allowed_mismatches
        && find_mirror(
            strings,
            i - 1,
            k + 1,
            if mismatches > 0 {
                allowed_mismatches - 1
            } else {
                allowed_mismatches
            },
        )
    {
        return true;
    }
    return false;
}

fn sum(c: &Vec<String>, mismatches: i32) -> usize {
    match (0..c.len() - 1)
        .into_iter()
        .position(|i| find_mirror(c, i as i32, i as i32 + 1, mismatches))
    {
        Some(i) => i + 1,
        None => usize::MAX,
    }
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let line: String = read_line(&input_path(day));

    let parts = vec![(1, 0), (2, 1)];
    for (part, mismatches) in parts.iter() {
        let c = parse(&line);
        let ans: usize = c
            .iter()
            .map(|(cols, rows)| {
                let r = sum(rows, *mismatches);
                let c = sum(cols, *mismatches);
                return if r < c { r * 100 } else { c * 1 };
            })
            .filter(|e| *e != usize::MAX)
            .sum();
        println!("Part {:?} {:?}", part, ans);
    }
}
