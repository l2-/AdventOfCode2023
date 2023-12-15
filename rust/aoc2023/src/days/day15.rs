use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;

fn custom_hash(s: &String) -> usize {
    let mut val = 0;
    for c in s.chars() {
        val += c as usize;
        val *= 17;
        val %= 256;
    }
    return val;
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    {
        for line in lines.iter() {
            let sum = string_split(line, ",")
                .iter()
                .fold(0, |acc, s| acc + custom_hash(s));
            println!("Part 1 {:?}", sum);
        }

        for line in lines.iter() {
            let mut map = HashMap::new() as HashMap<usize, Vec<(String, i64)>>;
            for s in string_split(line, ",").iter() {
                let split = string_split(&s.replace("-", "="), "=");
                let label = split[0].clone();
                let label_hash = custom_hash(&split[0]);

                // insert
                if split.len() > 1 && !split[1].is_empty() {
                    if !map.contains_key(&label_hash) { map.insert(label_hash, vec![]); }
                    let value = string_to_ints(&split[1])[0];
                    let v = map.get_mut(&label_hash).unwrap();
                    let i = v.iter().position(|(l, _)| l == &label);
                    if i.is_some() {
                        v[i.unwrap()] = (label, value);
                    } else {
                        v.push((label, value));
                    }
                }
                // remove
                else if map.contains_key(&label_hash) {
                    let v = map.get_mut(&label_hash).unwrap();
                    let i = v.iter().position(|(l, _)| l == &label);
                    if i.is_some() {
                        v.remove(i.unwrap());
                    }
                }
            }
            let sum = (0..256).filter(|i| map.contains_key(i)).fold(0, |acc, b| {
                let v = map.get(&b).unwrap();
                let sum = (0..v.len()).fold(0, |acc, i| acc + (i + 1) * v[i].1 as usize);
                return acc + sum * (b + 1);
            });
            println!("Part 2 {:?}", sum);
        }
    }
}
