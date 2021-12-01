use super::super::files;

pub fn run_a() {
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_1/input.txt",
    ));

    // map strings to numbers
    let depths: Vec<i32> = lines.iter().map(|x| x.parse().unwrap()).collect();

    // we will be counting the number of sequential increases in this variable
    let mut count: i32 = 0;

    // we will have to keep track of the previous depth to check if the depth
    // increased. we start with an "invalid" depth (a negative one), so we know
    // for sure the first depth will be an "increasing" one. later, we will have
    // to remove one from the count, to adjust for this strategy.
    let mut prev_depth: i32 = -1;

    // then we iterate through the depths, comparing the previous depth with
    // the current one. If the depth increases, we increase the count by one.
    // At the end, we re-assing the previous depth.
    for depth in depths {
        if depth > prev_depth {
            count += 1;
        }
        prev_depth = depth;
    }

    // remove the first increase.
    count -= 1;
    println!("{}", count);
}

pub fn run_b() {
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_1/input.txt",
    ));
    let depths: Vec<i32> = lines.iter().map(|x| x.parse().unwrap()).collect();

    let mut prev_depth: i32 = -1;
    let mut count: i32 = 0;

    // the solution to this exercise is pretty much the same, but instead of
    // looping through the depths as we were doing before, we loop through the
    // windows of size 3 of the depths, and sum the whole window when comparing.
    for window in depths.windows(3) {
        let depth: i32 = window.iter().sum();
        if depth > prev_depth {
            count += 1;
        }
        prev_depth = depth;
    }

    // remove the first increase, just like the previous exercise.
    count -= 1;
    println!("{}", count);
}
