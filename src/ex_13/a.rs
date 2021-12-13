use super::super::files;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

type Point = (u32, u32);

// a function that "folds" over a fixed Y value.
fn fold_y(points: &[Point], value: u32) -> Vec<Point> {
    // first it gets the total height. ie, the max value of Y.
    let height = points.iter().map(|&point| point.1).max().unwrap();

    // then, it iterates over the points. if the points are in the top half,
    // it leaves them there. otherwise, it maps their Y component to height - Y.
    let points: Vec<Point> = points.iter().map(|&point| {
        let (x, y) = point;
        if y > value {
            return (x,height - y)
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
        if x > value {
            return (width - x,y)
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

    // take the first fold and apply it
    let mut points = match folds.first().unwrap() {
        Fold::X(value) => fold_x(&points, *value),
        Fold::Y(value) => fold_y(&points, *value),
    };

    // then remove duplicated points. we have to sort it first, because dedup
    // only removes adjacent duplicates.
    points.sort_unstable();
    points.dedup();

    // finally count the points
    println!("{}", points.len());
}