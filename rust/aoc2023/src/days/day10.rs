use std::collections::VecDeque;

use super::super::dayx::*;
use super::super::ulitity::*;

const OPEN_LEFT: i32 = 0x01;
const OPEN_RIGHT: i32 = 0x02;
const OPEN_UP: i32 = 0x04;
const OPEN_DOWN: i32 = 0x08;

const MAPPINGC: [(char, i32); 8] = [
    ('|', OPEN_UP | OPEN_DOWN),
    ('-', OPEN_LEFT | OPEN_RIGHT),
    ('L', OPEN_UP | OPEN_RIGHT),
    ('J', OPEN_UP | OPEN_LEFT),
    ('7', OPEN_LEFT | OPEN_DOWN),
    ('F', OPEN_RIGHT | OPEN_DOWN),
    ('S', OPEN_UP | OPEN_DOWN | OPEN_LEFT | OPEN_RIGHT),
    ('.', 0),
];

const MAPPINGD: [(i32, i32, (i32, i32)); 4] = [
    (OPEN_UP, OPEN_DOWN, (0, -1)),
    (OPEN_DOWN, OPEN_UP, (0, 1)),
    (OPEN_LEFT, OPEN_RIGHT, (-1, 0)),
    (OPEN_RIGHT, OPEN_LEFT, (1, 0)),
];

const DXY: [Int2; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const KERNEL: [Int2; 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn is_connected(
    curr: (usize, usize),
    next: (usize, usize),
    width: usize,
    grid: &Vec<(char, i32)>,
) -> bool {
    let c1 = grid[curr.0 + curr.1 * width];
    let c2 = grid[next.0 + next.1 * width];
    let dxy = (next.0 as i32 - curr.0 as i32, next.1 as i32 - curr.1 as i32);
    let mapping1 = MAPPINGC.iter().find(|(c, _)| *c == c1.0).unwrap().1;
    let mapping2 = MAPPINGC.iter().find(|(c, _)| *c == c2.0).unwrap().1;
    let is_match = |&(l, r, _dxy)| {
        let cond1 = dxy == _dxy;
        let cond2 = mapping1 & l > 0 && mapping2 & r > 0;
        return cond1 && cond2;
    };
    return match dxy {
        (0, 1) => MAPPINGD.iter().any(is_match),
        (1, 0) => MAPPINGD.iter().any(is_match),
        (0, -1) => MAPPINGD.iter().any(is_match),
        (-1, 0) => MAPPINGD.iter().any(is_match),
        _ => false,
    };
}

fn fill(s: (usize, usize), width: usize, grid: &mut Vec<char>, bounds: AABB2) -> () {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(s);
    while !q.is_empty() {
        let (x, y) = q.pop_back().unwrap();
        grid[x + y * width] = '#';
        let candidates: Vec<_> = DXY
            .iter()
            .map(|(dx, dy)| ((x as i64 + *dx), (y as i64 + *dy)))
            .filter(|a @ (x, y)| {
                a.in_bounds(bounds) && grid[*x as usize + *y as usize * width] == '.'
            })
            .collect();
        candidates
            .iter()
            .for_each(|c| q.push_back((c.0 as usize, c.1 as usize)));
    }
}

fn trace_grid(s: (usize, usize), width: usize, grid: &mut Vec<(char, i32)>, bounds: AABB2) -> () {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(s);
    while !q.is_empty() {
        let curr = q.pop_back().unwrap();
        let curr1 = (curr.0 as i64, curr.1 as i64) as Int2;
        let candidates: Vec<(i64, i64)> = DXY
            .iter()
            .map(|(x, y)| (curr1.0 + x, curr1.1 + y))
            .filter(|a @ (x, y)| {
                a.in_bounds(bounds) && is_connected(curr, (*x as usize, *y as usize), width, grid)
            })
            .collect();
        let candidates1: Vec<(Int2, i32)> = candidates
            .iter()
            .map(|a| (a.to_owned(), grid[curr.0 + curr.1 * width].1 + 1))
            .collect();
        for ((x, y), u) in candidates1 {
            let next = (x as usize, y as usize);
            let u1 = grid[next.0 + next.1 * width].1;
            if u < u1 {
                grid[next.0 + next.1 * width].1 = u;
                q.push_back(next);
            }
        }
    }
}

fn construct_grid(lines: &Vec<String>) -> (Vec<(char, i32)>, (usize, usize), (usize, usize)) {
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = vec![('.', i32::MAX); width * height];
    let mut s = (0, 0);
    for y in 0..height {
        let line = lines[y].clone();
        for x in 0..width {
            let c = line.chars().nth(x).unwrap();
            grid[x + y * width] = (c, i32::MAX);
            if c == 'S' {
                s = (x, y);
                grid[x + y * width] = (c, 0);
            }
        }
    }
    return (grid, (width, height), s);
}

fn zero_out_useless(grid: &Vec<(char, i32)>) -> Vec<(char, i32)> {
    grid.iter()
        .map(|(c, u)| (if *u < i32::MAX { c.to_owned() } else { '.' }, u.to_owned()))
        .collect()
}

// pass a traced grid
fn construct_wide_grid(
    grid: &Vec<(char, i32)>,
    width: usize,
    height: usize,
) -> (Vec<char>, (usize, usize)) {
    let nwidth = width * 2 + 2;
    let nheight: usize = height * 2 + 2;
    let bounds = ((0i64, 0i64), (width as i64 - 1, height as i64 - 1)) as AABB2;
    let mut ngrid = vec!['.'; nwidth * nheight];
    for y in 0..height {
        for x in 0..width {
            ngrid[(x * 2 + 1) + (y * 2 + 1) * nwidth] = grid[x + y * width].0;
        }
    }

    // add connections
    for y in 0..height {
        for x in 0..width {
            let _x = x as i64;
            let _y = y as i64;
            let candidates: Vec<_> = DXY
                .iter()
                .map(|(__x, __y)| ((_x + __x, _y + __y), (__x, __y)))
                .filter(|(a @ (__x, __y), _)| {
                    a.in_bounds(bounds)
                        && is_connected((x, y), (*__x as usize, *__y as usize), width, &grid)
                })
                .collect();
            for (_, (dx, dy)) in candidates {
                let dx = *dx;
                let dy = *dy;
                let x = (_x * 2 + 1 + dx) as usize;
                let y: usize = (_y * 2 + 1 + dy) as usize;
                ngrid[x + y * nwidth] = '*';
            }
        }
    }
    return (ngrid, (nwidth, nheight));
}

fn count_free_pixels(
    grid: &mut Vec<(char, i32)>,
    width: usize,
    height: usize,
    ngrid: &Vec<char>,
    nwidth: usize,
) -> i32 {
    for _y in 0..height {
        for _x in 0..width {
            let x = _x * 2 + 1;
            let y = _y * 2 + 1;
            let c = ngrid[x + y * nwidth];
            if c != '.' {
                continue;
            }
            if KERNEL.iter().all(|(dx, dy)| {
                let x = (x as i64 + dx) as usize;
                let y = (y as i64 + dy) as usize;
                return ngrid[x + y * nwidth] == '.';
            }) {
                grid[_x + _y * width] = ('I', grid[_x + _y * width].1);
            }
        }
    }
    return grid
        .iter()
        .fold(0, |acc, (c, _)| acc + if *c == 'I' { 1 } else { 0 });
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    {
        let (mut grid, (width, height), s) = construct_grid(&lines);
        let bounds = ((0i64, 0i64), (width as i64 - 1, height as i64 - 1)) as AABB2;
        trace_grid(s, width, &mut grid, bounds);
        let ans = grid
            .iter()
            .map(|e| e.1)
            .filter(|e| *e != i32::MAX)
            .reduce(|acc, e| std::cmp::max(acc, e))
            .unwrap();
        println!("Part 1 {:?}", ans);
        // print_single_char_matrix(&grid.iter().map(|(e, _)| *e).collect(), width, height);

        // this step replaces all the chars that are not connected to S with '.' (empty)
        grid = zero_out_useless(&grid);
        // println!("");
        // print_single_char_matrix(&grid.iter().map(|(e, _)| *e).collect(), width, height);

        // construct the bigger grid so we can mark connections between pipes
        let (mut ngrid, (nwidth, nheight)) = construct_wide_grid(&grid, width, height);
        let nbounds = ((0i64, 0i64), (nwidth as i64 - 1, nheight as i64 - 1)) as AABB2;
        // println!("");
        // print_single_char_matrix(&ngrid, nwidth, nheight);

        // flood fill from 0, 0. this works because our bigger grid has an boundary of '.' (empties)
        fill((0, 0), nwidth, &mut ngrid, nbounds);
        // println!("");
        // print_single_char_matrix(&ngrid, nwidth, nheight);

        // count the number of 'I' (the empties after the fill). we need both grids since we don't want to count the empties we added for the bigger grid
        let ans = count_free_pixels(&mut grid, width, height, &ngrid, nwidth);
        println!("Part 2 {:?}", ans);
        // print_single_char_matrix(&grid.iter().map(|(e, _)| *e).collect(), width, height);
    }
}
