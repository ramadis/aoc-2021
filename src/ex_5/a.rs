use super::super::files;
use std::str::FromStr;
use std::string::ParseError;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash)]
struct Point {
    x: u32,
    y: u32,
}

// this allows us to use str.parse() with the Point type.
impl FromStr for Point {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (raw_x, raw_y) = raw.split_once(',').unwrap();

        let x: u32 = raw_x.parse().unwrap();
        let y: u32 = raw_y.parse().unwrap();

        Ok(Point { x, y })
    }
}

// this is required for the hashmap trait
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

type Line = (Point, Point);

fn is_line_diagonal(line: &Line) -> bool {
    let (p1, p2) = line;
    !(p1.x == p2.x || p1.y == p2.y)
}

fn get_points_in_line(line: &Line) -> Vec<Point> {
    let (p1, p2) = line;
    let mut vec = vec![];

    // just ignore diagonals for the sake of this exercise
    if is_line_diagonal(line) {
        return vec;
    }

    // get the points within the line (i think this doesn't work for diagonal lines lol)
    for x in cmp::min(p1.x, p2.x)..(cmp::max(p1.x, p2.x) + 1) {
        for y in cmp::min(p1.y, p2.y)..(cmp::max(p1.y, p2.y) + 1) {
            vec.push(Point { x, y });
        }
    }

    vec
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_5/input.txt",
    ));

    // we can do all this parsing like this because we know the input will be well-formed.
    let lines: Vec<Line> = raw_lines.iter().map(|raw_line| {
        let parts: Vec<&str> = raw_line.split_whitespace().collect();
        let beggining: Point = parts[0].parse().unwrap();
        let end = parts[2].parse().unwrap();
        let line: Line = (beggining, end);
        line
    }).collect();

    // we create a map from points, to all the lines that contain that point
    let mut points_map:HashMap<Point, Vec<&Line>> = HashMap::new();
    for line in lines.iter() {
        let points = get_points_in_line(line);
        for point in points.iter() {
            if points_map.contains_key(point) {
                points_map.get_mut(point).unwrap().push(line);
            } else {
                points_map.insert(*point, vec![line]);
            }
        }
    }

    // then, we just count how many entries with # of lines > 1 there are in the map
    let mut count_overlapped = 0;
    for (_, lines) in points_map.iter() {
        if lines.len() > 1 {
            count_overlapped += 1;
        }
    }

    println!("{:?}", count_overlapped);
}