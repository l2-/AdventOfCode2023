use regex::Regex;
use std::{*, collections::*};
pub type Int2 = (i64, i64);
pub type Int3 = (i64, i64, i64);
pub type AABB2 = (Int2, Int2);
pub type AABB3 = (Int3, Int3);

pub fn read_lines(file_path: &str) -> Vec<String> {
    let res = fs::read_to_string(file_path);
    return match res {
        Ok(_str) => _str.lines().map(|l| String::from(l)).collect::<Vec<_>>(),
        Err(_) => panic!("No file found for {}", file_path),
    };
}

pub fn read_line(file_path: &str) -> String {
    let res = fs::read_to_string(file_path);
    return match res {
        Ok(_str) => _str.to_string(),
        Err(_) => panic!("No file found for {}", file_path),
    };
}

pub fn transpose_strings(s: &Vec<String>) -> Vec<String> {
    let mut lines = vec![] as Vec<String>;
    for y in 0..s.len() {
        for x in 0..s[y].len() {
            let __s = s[y].chars().nth(x).unwrap().to_string();
            let _s = if x < lines.len() {
                lines[x].to_string() + &__s
            } else {
                __s
            };
            if x < lines.len() {
                lines[x] = _s;
            } else {
                lines.push(_s)
            };
        }
    }
    return lines;
}

pub fn string_split(_str: &str, pattern: &str) -> Vec<String> {
    return _str
        .split(pattern)
        .map(|f| String::from(f))
        .collect::<Vec<_>>();
}

pub fn string_to_ints(_str: &str) -> Vec<i64> {
    let mut res = vec![];
    let re = Regex::new(r"-?\d+").unwrap();
    for m in re.find_iter(_str) {
        match m.as_str().parse::<i64>() {
            Ok(i) => res.push(i),
            Err(_) => (),
        }
    }
    return res;
}

// returns (a, b) in y = ax + b given 2 points
pub fn line_from_points(x1: i64, y1: i64, x2: i64, y2: i64) -> Int2 {
    let a = (y2 - y1) / (x2 - x1);
    let b = y1 - x1 * a;
    return (a, b);
}

pub fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    return (x1 - x2).abs() + (y1 - y2).abs();
    // return distance(x1, x2) + distance(y1, y2);
}

pub fn reduce_line_segments(_segments: &Vec<Int2>) -> Vec<Int2> {
    let mut segments = _segments.clone();
    segments.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    let mut result: Vec<Int2> = vec![];
    if segments.len() < 1 {
        return result;
    }
    let mut start_segment = segments[0].0;
    let mut end_segment = segments[0].1;
    for (start, end) in segments.into_iter() {
        if start > end_segment {
            result.push((start_segment, end_segment));
            start_segment = start;
        }
        end_segment = std::cmp::max(end, end_segment);
    }
    result.push((start_segment, end_segment));
    return result;
}

pub fn intersect((a1, b1): Int2, (a2, b2): Int2, x_out: &mut i64, y_out: &mut i64) -> bool {
    if a1 - a2 == 0 {
        return false;
    }
    *x_out = (b2 - b1) / (a1 - a2);
    *y_out = a1 * (*x_out) + b1;
    return true;
}

// inclusive
pub fn distance(a: i64, b: i64) -> i64 {
    return i64::abs(a - b) + 1;
}

pub trait AABB3t {
    fn grow(&self, point: Int3) -> Self;
}

impl AABB3t for AABB3 {
    fn grow(&self, point: Int3) -> Self {
        (
            self.0.element_wise_min(point),
            self.1.element_wise_max(point),
        )
    }
}

pub trait AABB2t {
    fn grow(&self, point: Int2) -> Self;
}

impl AABB2t for AABB2 {
    fn grow(&self, point: Int2) -> Self {
        (
            self.0.element_wise_min(point),
            self.1.element_wise_max(point),
        )
    }
}

pub trait Bounds1d {
    fn in_bounds(&self, _min: i64, _max: i64) -> bool;
}
impl Bounds1d for i64 {
    fn in_bounds(&self, _min: i64, _max: i64) -> bool {
        return self >= &_min && self <= &_max;
    }
}
pub trait Bounds2d {
    fn in_bounds(&self, aabb: AABB2) -> bool;
}
impl Bounds2d for Int2 {
    fn in_bounds(&self, ((min_x, min_y), (max_x, max_y)): AABB2) -> bool {
        return self.0.in_bounds(min_x, max_x) && self.1.in_bounds(min_y, max_y);
    }
}
pub trait Bounds3d {
    fn in_bounds(&self, aabb: AABB3) -> bool;
}
impl Bounds3d for Int3 {
    fn in_bounds(&self, (_min, _max): AABB3) -> bool {
        return self.0.in_bounds(_min.0, _max.0)
            && self.1.in_bounds(_min.1, _max.1)
            && self.2.in_bounds(_min.2, _max.2);
    }
}
pub trait Distance2d {
    fn manhattan_distance(&self, other: Int2) -> i64;
}
impl Distance2d for Int2 {
    fn manhattan_distance(&self, (x2, y2): Int2) -> i64 {
        manhattan_distance(self.0, self.1, x2, y2)
    }
}
pub trait Maths3i64 {
    fn add(&self, other: Int3) -> Int3;
    fn sub(&self, other: Int3) -> Int3;
    fn mul(&self, scalar: i64) -> Int3;
    fn element_wise_min(&self, other: Int3) -> Int3;
    fn element_wise_max(&self, other: Int3) -> Int3;
}
impl Maths3i64 for Int3 {
    fn add(&self, (x, y, z): Int3) -> Int3 {
        (self.0 + x, self.1 + y, self.2 + z)
    }
    fn sub(&self, (x, y, z): Int3) -> Int3 {
        (self.0 - x, self.1 - y, self.2 - z)
    }
    fn element_wise_min(&self, (x, y, z): Int3) -> Int3 {
        return (self.0.min(x), self.1.min(y), self.2.min(z));
    }
    fn element_wise_max(&self, (x, y, z): Int3) -> Int3 {
        return (self.0.max(x), self.1.max(y), self.2.max(z));
    }
    fn mul(&self, scalar: i64) -> Int3 {
        (self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}
pub trait Maths2i64 {
    fn add(&self, other: Int2) -> Int2;
    fn sub(&self, other: Int2) -> Int2;
    fn mul(&self, scalar: i64) -> Int2;
    fn element_wise_min(&self, other: Int2) -> Int2;
    fn element_wise_max(&self, other: Int2) -> Int2;
    fn determinant(&self, other: Int2) -> i64;
}
impl Maths2i64 for Int2 {
    fn add(&self, (x, y): Int2) -> Int2 {
        (self.0 + x, self.1 + y)
    }
    fn sub(&self, (x, y): Int2) -> Int2 {
        (self.0 - x, self.1 - y)
    }
    fn element_wise_min(&self, (x, y): Int2) -> Int2 {
        return (self.0.min(x), self.1.min(y));
    }
    fn element_wise_max(&self, (x, y): Int2) -> Int2 {
        return (self.0.max(x), self.1.max(y));
    }

    fn mul(&self, scalar: i64) -> Int2 {
        (self.0 * scalar, self.1 * scalar)
    }
    fn determinant(&self, other: Int2) -> i64 {
        self.0 * other.1 - self.1 * other.0
    }
}

pub fn print_single_digit_matrix(matrix: &Vec<i64>, width: usize, height: usize) -> () {
    for y in 0..height {
        let mut s: String = String::from("");
        for x in 0..width {
            let mut c = String::from("*");
            let d = matrix[x + y * width];
            if d.in_bounds(0, 9) {
                c = d.to_string();
            }
            s += &c;
        }
        println!("{0}", s);
    }
}
pub fn print_single_char_matrix(matrix: &Vec<char>, width: usize, height: usize) -> () {
    for y in 0..height {
        let mut s: String = String::from("");
        for x in 0..width {
            let d = matrix[x + y * width];
            s += &String::from(d);
        }
        println!("{0}", s);
    }
}

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

fn _gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    return n;
}

pub fn gcd(nums: &Vec<u64>) -> u64 {
    return nums.iter().copied().reduce(|acc, e| _gcd(acc, e)).unwrap();
}

pub fn lcm(nums: &Vec<u64>) -> u64 {
    let gcd = gcd(nums);
    return nums
        .iter()
        .copied()
        .reduce(|acc, e| acc * (e / gcd))
        .unwrap();
}

pub fn get_day(filename: &String) -> i32 {
    string_to_ints(&filename)[0] as i32
}

pub fn fill2d<T: Clone + PartialEq>(
    grid: &mut Vec<T>,
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
    fill_value: T,
    filled_value: T,
) {
    let mut q = VecDeque::new() as VecDeque<(usize, usize)>;
    let bounds = ((0i64, 0i64), (width as i64 - 1, height as i64 - 1));
    q.push_back((start_x, start_y));
    let directions = [(-1i64, 0i64), (0, -1), (1, 0), (0, 1)];
    while let Some((x, y)) = q.pop_back() {
        grid[x + y * width] = fill_value.clone();
        directions.iter().for_each(|&(dx, dy)| {
            let next = (x as i64 + dx, y as i64 + dy);
            if !next.in_bounds(bounds) {
                return;
            }
            let next = (next.0 as usize, next.1 as usize);
            if grid[next.0 + next.1 * width] == fill_value
                || grid[next.0 + next.1 * width] == filled_value
            {
                return;
            }
            q.push_back(next);
        });
    }
}

pub fn fill3d<T: Clone + PartialEq>(
    grid: &mut Vec<T>,
    width: usize,
    height: usize,
    depth: usize,
    start_x: usize,
    start_y: usize,
    start_z: usize,
    fill_value: T,
    filled_value: T,
) {
    let mut q = VecDeque::new() as VecDeque<(usize, usize, usize)>;
    let bounds = (
        (0i64, 0i64, 0i64),
        (width as i64 - 1, height as i64 - 1, depth as i64 - 1),
    );
    q.push_back((start_x, start_y, start_z));
    let directions: [Int3; 6] = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    while let Some((x, y, z)) = q.pop_back() {
        grid[x + y * width + z * width * height] = fill_value.clone();
        directions.iter().for_each(|&(dx, dy, dz)| {
            let next = (x as i64 + dx, y as i64 + dy, z as i64 + dz);
            if !next.in_bounds(bounds) {
                return;
            }
            let next = (next.0 as usize, next.1 as usize, next.2 as usize);
            if grid[next.0 + next.1 * width + next.2 * width * height] == fill_value
                || grid[next.0 + next.1 * width + next.2 * width * height] == filled_value
            {
                return;
            }
            q.push_back(next);
        });
    }
}

pub fn hex_to_int(s: &String) -> i64 {
    i64::from_str_radix(&s.replace("#", ""), 16).unwrap()
}

// area of polygon enclosed by lines, lines need to be enclosing an area and in the right order
pub fn trapezoid_formula<'a, T>(lines: &mut T) -> i64
where
    T: Iterator<Item = &'a Int2>,
{
    let current = lines.next();
    if current.is_none() {
        return 0;
    }
    let mut current = *current.unwrap();
    let mut sum = 0;
    while let Some(&next) = lines.next() {
        let area = current.determinant(next);
        sum += area;
        current = next;
    }
    return sum.abs() / 2;
}
