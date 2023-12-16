use std::collections::HashSet;
use std::collections::VecDeque;

use super::super::dayx::*;
use super::super::ulitity::*;

fn track_beam(
    seen: &mut HashSet<(i32, i32, i32, i32)>,
    set: &mut HashSet<(i32, i32)>,
    grid: &Vec<u8>,
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) {
    let mut q: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();
    q.push_back((x, y, dx, dy));
    while !q.is_empty() {
        let (x, y, dx, dy) = q.pop_back().unwrap();
        if seen.contains(&(x, y, dx, dy)) {
            continue;
        }
        if ((x as i64, y as i64) as Int2)
            .in_bounds(((0i64, 0i64), (width as i64 - 1, height as i64 - 1)))
        {
            seen.insert((x, y, dx, dy));
            set.insert((x, y));
        }

        let next_x = x + dx;
        let next_y = y + dy;
        if !((next_x as i64, next_y as i64) as Int2)
            .in_bounds(((0i64, 0i64), (width as i64 - 1, height as i64 - 1)))
        {
            continue;
        }
        let next = grid[next_x as usize + next_y as usize * width];
        match (next, dx.abs(), dy.abs()) {
            (1, _, _) => q.push_back((next_x, next_y, dy, dx)),
            (2, _, _) => q.push_back((next_x, next_y, dy * -1, dx * -1)),

            (3, _, 1) => {
                q.push_back((next_x, next_y, 1, 0));
                q.push_back((next_x, next_y, -1, 0));
            }
            (4, 1, _) => {
                q.push_back((next_x, next_y, 0, -1));
                q.push_back((next_x, next_y, 0, 1));
            }
            _ => q.push_back((next_x, next_y, dx, dy)),
        }
    }
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let width = lines[0].len();
    let height = lines.len();
    let mut grid = vec![0u8; width * height] as Vec<u8>;
    for y in 0..height {
        for x in 0..width {
            grid[x + y * width] = match lines[y].chars().nth(x).unwrap() {
                '|' => 4,
                '-' => 3,
                '/' => 2,
                '\\' => 1,
                _ => 0,
            }
        }
    }
    let parts = vec![
        (1, vec![(-1, 0, 1, 0)]),
        (
            2,
            (0..width as i32)
                .map(|x| (x, -1, 0, 1))
                .chain((0..width as i32).map(|x| (x, height as i32, 0, -1)))
                .chain((0..height as i32).map(|y| (-1, y, 0, 1)))
                .chain((0..height as i32).map(|y| (width as i32, y as i32, 0, -1)))
                .collect::<Vec<(i32, i32, i32, i32)>>(),
        ),
    ];

    for (part, starts) in parts.iter() {
        let mut _max = 0;
        for (x, y, dx, dy) in starts.iter() {
            let mut seen = HashSet::new() as HashSet<(i32, i32, i32, i32)>;
            let mut set = HashSet::new() as HashSet<(i32, i32)>;
            track_beam(&mut seen, &mut set, &grid, width, height, *x, *y, *dx, *dy);
            _max = std::cmp::max(_max, set.len());
        }
        println!("Part {:?} {:?}", part, _max);
    }
}
