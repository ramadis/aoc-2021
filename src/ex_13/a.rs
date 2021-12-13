use super::super::files;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

type Point = (u32, u32);

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_13/example.txt",
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
}