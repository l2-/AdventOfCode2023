use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use super::super::dayx::*;
use super::super::ulitity::*;

static DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Int2,
    direction: Int2,
    in_a_row: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(tile_costs: &Vec<usize>, width: usize, height: usize, start: Int2, goal: Int2, min_in_a_row: i64, max_in_a_row: i64) -> usize {
    let mut seen: HashSet<(Int2, Int2, i64)> = HashSet::new();
    let mut q = BinaryHeap::new() as BinaryHeap<State>;
    let get_cost = |(x, y)| tile_costs[x as usize + y as usize * width];
    q.push(State {
        cost: 0,
        position: start,
        direction: (0i64, 0i64),
        in_a_row: 0,
    });
    while let Some(State {
        cost,
        position: position @ (x, y),
        direction: direction @ (dx, dy),
        in_a_row
    }) = q.pop()
    {
        if position == goal && in_a_row >= min_in_a_row {
            return cost;
        }
        if seen.contains(&(position, direction, in_a_row)) {
            continue;
        }
        seen.insert((position, direction, in_a_row));
        if in_a_row < max_in_a_row && (dx != 0 || dy != 0) {
            let next_position = (x + dx, y + dy);
            if next_position.in_bounds(((0i64, 0i64), (width as i64 - 1, height as i64 - 1))) {
                q.push(State { cost: get_cost(next_position) + cost, position: next_position, direction, in_a_row: in_a_row + 1 })
            }
        }
        if in_a_row >= min_in_a_row || (dx == 0 && dy == 0) {
            for &(_dx, _dy) in DIRECTIONS.iter() {
                let next_direction = (_dx, _dy);
                if next_direction == direction || (_dx == -dx && _dy == -dy) { continue; }
                let next_position = (x + _dx, y + _dy);
                if next_position.in_bounds(((0i64, 0i64), (width as i64 - 1, height as i64 - 1))) {
                    q.push(State { cost: get_cost(next_position) + cost, position: next_position, direction: next_direction, in_a_row: 1 })
                }
            }
        }
    }
    return usize::MAX;
}

pub fn day() {
    let day = get_day(&file!().to_owned());
    print_day(day);
    let lines: Vec<String> = read_lines(&input_path(day));

    let parts = vec![(1, 0, 3), (2, 4, 10)];
    for &(part, min_in_a_row, max_in_a_row) in parts.iter() {
        let width = lines[0].len();
        let height = lines.len();
        let tile_costs: Vec<usize> = (0..height)
            .flat_map(|y| {
                (0..width)
                    .map(move |x| (x, y))
                    .map(|(x, y)| (lines[y].chars().nth(x).unwrap() as i32 - '0' as i32) as usize)
            })
            .collect();
        let start = (0i64, 0i64) as Int2;
        let goal = (width as i64 - 1, height as i64 - 1) as Int2;
        let cost = dijkstra(&tile_costs, width, height, start, goal, min_in_a_row, max_in_a_row);
        println!("Part {:?} {:?}", part, cost);
    }
}
