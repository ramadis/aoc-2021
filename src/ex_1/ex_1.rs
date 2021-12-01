use super::super::files;

pub fn run_a() {
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_1/a.txt",
    ));
    let depths: Vec<i32> = lines.iter().map(|x| x.parse().unwrap()).collect();

    let mut prev_depth: i32 = -1;
    let mut count: i32 = 0;
    for depth in depths {
        if depth > prev_depth {
            count += 1;
        }
        prev_depth = depth;
    }

    // remove the first increase
    count -= 1;
    println!("{}", count);
}

pub fn run_b() {
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_1/a.txt",
    ));
    let depths: Vec<i32> = lines.iter().map(|x| x.parse().unwrap()).collect();

    let mut prev_depth: i32 = -1;
    let mut count: i32 = 0;
    for window in depths.windows(3) {
        let depth: i32 = window.iter().sum();
        if depth > prev_depth {
            count += 1;
        }
        prev_depth = depth;
    }

    // remove the first increase
    count -= 1;
    println!("{}", count);
}
