use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = 9;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let parts = vec![(1, true), (2, false)];
    for (part, atend) in parts
    {
        let mut sum = 0;
        for line in lines.iter() {
            let v = string_to_ints(line);
            let mut map: HashMap<i64, Vec<i64>> = HashMap::new();
            let mut i = 0;
            map.insert(i, v);
            i +=1 ;
            loop {
                let lastv = map.get(&(i - 1)).unwrap();
                if lastv.len() < 2 || lastv.iter().all(|v| *v == 0) { break; }
                let v = (0..lastv.len() - 1).map(|k| lastv[k + 1] - lastv[k]).collect();
                map.insert(i, v);
                i += 1;
            }
            let mut lastchange = 0;
            for i in (0 ..= i - 2).rev() {
                let v = map.get_mut(&i).unwrap();
                if atend {
                    v.push(v.last().unwrap() + lastchange);
                    lastchange = *v.last().unwrap();
                } else {
                    v.insert(0, v.first().unwrap() - lastchange);
                    lastchange = *v.first().unwrap();
                }
            }
            sum += if atend { map.get(&0).unwrap().last().unwrap() } else { map.get(&0).unwrap().first().unwrap() };
        }

        println!("Part {} {}", part, sum);
    }
}