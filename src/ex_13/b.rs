use super::super::files;
use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

type Point = (u32, u32);

// a function that prints the points
fn print_points(points: &[Point]) {
    let height = points.iter().map(|&point| point.1).max().unwrap();
    let width = points.iter().map(|&point| point.0).max().unwrap();
    let mut set = HashSet::new();

    for &point in points.iter() {
         set.insert(point);
    }

    for y in 0..height+1 {
        for x in 0..width+1 {
            if set.contains(&(x, y)) {
                print!("❚");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

// a function that "folds" over a fixed Y value.
fn fold_y(points: &[Point], value: u32) -> Vec<Point> {
    // first it gets the total height. ie, the max value of Y.
    let height = points.iter().map(|&point| point.1).max().unwrap();

    // then, it iterates over the points. if the points are in the top half,
    // it leaves them there. otherwise, it maps their Y component to height - Y.
    // one gotcha though. If the height is even, then there's no "middle" on which
    // to fold. In that case, we have to offset the points by 1 place :_).
    let points: Vec<Point> = points.iter().map(|&point| {
        let (x, y) = point;
        let diff = if height % 2 == 0 { height - y } else { height - y + 1};
        if y > value {
            return (x, diff);
        }

        point
    }).collect();
    points
}

// analogous to fold_y, but for X.
fn fold_x(points: &[Point], value: u32) -> Vec<Point> {
    let width = points.iter().map(|&point| point.0).max().unwrap();
    let points: Vec<Point> = points.iter().map(|&point| {
        let (x, y) = point;
        let diff = if width % 2 == 0 { width - x } else { width - x + 1};
        if x > value {
            return (diff, y);
        }

        point
    }).collect();
    points
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_13/input.txt",
    ));

    // parse the list of folds and points
    let mut folds: Vec<Fold> = vec![];
    let mut points: Vec<Point> = vec![];
    for line in raw_lines {
        if line.starts_with("fold") {
            let (first, value) = line.split_once("=").unwrap();
            let value: u32 = value.parse().unwrap();

            if first.ends_with("y") {
                folds.push(Fold::Y(value))
            } else if first.ends_with("x") {
                folds.push(Fold::X(value))
            }

            continue;
        }

        if !line.contains(",") {
            continue;
        }

        let (x, y) = line.split_once(",").unwrap();
        let (x, y): Point = (x.parse().unwrap(), y.parse().unwrap());
        points.push((x, y));
    }

    // iterate over the folds, applying it over the points obtained in the
    // previous step (basically, a reduce)
    for fold in folds {
        points = match fold {
            Fold::X(value) => fold_x(&points, value),
            Fold::Y(value) => fold_y(&points, value),
        };
    }
    
    // print the points. no need to dedupe in this case, since repeated points are
    // gonna be ignored.
    print_points(&points);
}