
use std::collections::HashSet;

use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = 3;
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));
    let width = lines[0].len();
    let height = lines.len();
    let mut field= vec![0; width * height];
    for i in 0..lines.len() {
        let line = &lines[i];
        let mut ii = 0;
        let mut flag = false;
        let ints = string_to_ints(line);
        for k in 0..line.len() {
            match line.chars().nth(k).unwrap() {
                '0'..='9' => { field[k + width * i] = ints[ii]; flag = true },
                _ => {
                    if flag {
                        ii+=1;
                        flag = false;
                    }
                } 
            }
        }
    }
    println!("{} {} {} {} {}", field[0+0], field[2+0], field[3+0], field[5+0], field[2+2*width]);

    let insetts: Vec<i64> = vec![
        -1 -1 * width as i64, 0 -1 * width as i64, 1 -1 * width as i64,
        -1 + 0 * width as i64, 1 + 0 * width as i64,
        -1 + 1 * width as i64, 0 + 1 * width as i64, 1 + 1 * width as i64,
    ];

    let mut sum = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        for k in 0..line.len() {
            match line.chars().nth(k).unwrap() {
                '0'..='9' => (),
                '.' => (),
                _ => {
                    let mut seen: HashSet<i64> = HashSet::new();
                    let ii = k + i * width;
                    for insett in insetts.iter() {
                        let val = field[(ii as i64 + insett) as usize];
                        seen.insert(val);
                    }
                    sum += seen.iter().sum::<i64>();
                }
            }
        }
    }
    println!("Part 1 {}", sum);
    
    let mut sum = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        for k in 0..line.len() {
            match line.chars().nth(k).unwrap() {
                '0'..='9' => (),
                '.' => (),
                '*' => {
                    let mut seen: HashSet<i64> = HashSet::new();
                    let ii = k + i * width;
                    for insett in insetts.iter() {
                        let val = field[(ii as i64 + insett) as usize];
                        seen.insert(val);
                    }
                    if seen.len() == 3 { // 3 instead of 2 because we add the number 0
                        sum += seen.iter().fold(1i64, |cur, nxt| std::cmp::max(1, *nxt) * cur);
                    }
                },
                _ => ()
            }
        }
    }
    println!("Part 2 {}", sum);
}