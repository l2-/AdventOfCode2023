use std::collections::HashSet;

use super::super::dayx::*;
use super::super::ulitity::*;

fn construct_grid(lines: &Vec<String>) -> (Vec<char>, (usize, usize)) {
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = vec!['.'; width * height];
    for y in 0..height {
        for x in 0..width {
            grid[x + y * width] = lines[y].chars().nth(x).unwrap();
        }
    }
    return (grid, (width, height));
}

fn free_columns(grid: &Vec<char>, width: usize, height: usize) -> Vec<usize> {
    (0..width)
        .into_iter()
        .filter(|x| (0..height).into_iter().all(|y| grid[x + y * width] == '.'))
        .collect()
}

fn free_rows(grid: &Vec<char>, width: usize, height: usize) -> Vec<usize> {
    (0..height)
        .into_iter()
        .filter(|y| (0..width).into_iter().all(|x| grid[x + y * width] == '.'))
        .collect()
}

fn pairs_from_small_grid(
    grid: &Vec<char>,
    width: usize,
    height: usize,
    free_cols: &Vec<usize>,
    free_rows: &Vec<usize>,
    offset: i32,
) -> HashSet<(Int2, Int2)> {
    let mut locs: Vec<(usize, usize)> = vec![];
    let mut yoff = 0;
    for y in 0..height {
        if free_rows.contains(&y) {
            yoff += offset as usize;
        }
        let mut xoff = 0;
        for x in 0..width {
            if free_cols.contains(&x) {
                xoff += offset as usize;
            }
            let c = grid[x + y * width];
            if c != '.' {
                let x = x + xoff;
                let y = y + yoff;
                locs.push((x as usize, y as usize));
            }
        }
    }

    let mut pairs = HashSet::new();
    locs.iter().for_each(|&loc1 @ (x1, y1)| {
        locs.iter().for_each(|&loc2 @ (x2, y2)| {
            let pair = (
                (x1 as i64, y1 as i64) as Int2,
                (x2 as i64, y2 as i64) as Int2,
            );
            if loc1 != loc2 && !pairs.contains(&(pair.1, pair.0)) {
                pairs.insert(pair);
            }
        })
    });
    return pairs;
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let parts = vec![(1, 1), (1, 1000000-1)];
    for &(part, multiplier) in parts.iter() {
        let (grid, (width, height)) = construct_grid(&lines);
        let freecolumns = free_columns(&grid, width, height);
        let freerows = free_rows(&grid, width, height);
        let pairs = pairs_from_small_grid(&grid, width, height, &freecolumns, &freerows, multiplier);
        let distances: Vec<_> = pairs
            .iter()
            .map(|&(i1, i2)| (i1, i2, i1.manhattan_distance(i2)))
            .collect();
        let sum = distances.iter().fold(0i64, |acc, &(_, _, d)| acc + d);
        println!("Part {:?} {:?}", part, sum);
    }
}
