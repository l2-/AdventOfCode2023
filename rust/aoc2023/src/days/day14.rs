use std::collections::HashMap;

use super::super::dayx::*;
use super::super::ulitity::*;

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    {
        let width = lines[0].len();
        let height = lines.len();
        let mut _grid = vec![1i64; width * height] as Vec<i64>;
        for y in 0..height {
            for x in 0..width {
                _grid[x + y * width] = match lines[y].chars().nth(x).unwrap() {
                    '#' => 1,
                    'O' => 2,
                    _ => 0,
                }
            }
        }
        let directions = vec![
            (
                width,
                0 - 1,
                0,
                std::cmp::Ordering::Less,
                1,
                (0..height).collect::<Vec<usize>>(),
                (0..width).collect::<Vec<usize>>(),
            ),
            (
                height,
                0 - 1,
                1,
                std::cmp::Ordering::Less,
                1,
                (0..width).collect::<Vec<usize>>(),
                (0..height).collect::<Vec<usize>>(),
            ),
            (
                width,
                height as i32,
                0,
                std::cmp::Ordering::Greater,
                -1,
                (0..height).rev().collect::<Vec<usize>>(),
                (0..width).rev().collect::<Vec<usize>>(),
            ),
            (
                height,
                width as i32,
                1,
                std::cmp::Ordering::Greater,
                -1,
                (0..width).rev().collect::<Vec<usize>>(),
                (0..height).rev().collect::<Vec<usize>>(),
            ),
        ];

        let parts = vec![(1, 1), (2, 1000000000 * 4)];
        for &(part, cycles) in parts.iter() {
            let mut grid = _grid.clone();
            // rust supports hashing vec<i64> lol?
            let mut map = HashMap::new() as HashMap<Vec<i64>, usize>;
            let mut cycle = 0;
            while cycle < cycles {
                let (d1, initial, i, ord, inc, iter1, iter2) =
                    &directions[cycle % directions.len()];
                let mut last_stop: Vec<i32> = vec![*initial; *d1];
                for &y in iter1.iter() {
                    for &x in iter2.iter() {
                        let c = grid[x + y * d1];
                        let val_a = vec![x, y][*i]; //x
                        let val_b = vec![x, y][1 - i]; //y
                        let val_c: usize = (last_stop[val_a] + inc) as usize;
                        if c == 2 {
                            if val_c.cmp(&val_b) == *ord {
                                grid[if *i == 0 {
                                    val_a + val_c * d1
                                } else {
                                    val_c + val_a * d1
                                }] = 2;
                                grid[x + y * d1] = 0;
                            }
                            last_stop[val_a] += inc;
                        }
                        if c == 1 {
                            last_stop[val_a] = val_b as i32;
                        }
                    }
                }
                if !map.contains_key(&grid) {
                    map.insert(grid.clone(), cycle);
                } else {
                    let &c1 = map.get(&grid).unwrap();
                    let l = cycle as i64 - c1 as i64;
                    let loop_count = (cycles - cycle) as i64 / l;
                    let next_cycle = cycle as i64 + loop_count * l;
                    cycle = (next_cycle + 1) as usize;
                    continue;
                }
                cycle += 1;
            }
            let sum: usize = (0..height)
                .flat_map(|y| {
                    (0..width)
                        .map(move |x| (x + y * width, y))
                        .filter(|(i, _)| grid[*i] == 2)
                        .map(|(_, y)| height - y)
                })
                .sum();
            println!("Part {:?} {:?}", part, sum);
        }
    }
}
