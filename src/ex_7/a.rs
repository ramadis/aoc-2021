use super::super::files;

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_7/input.txt",
    ));

    // then we parse them
    let positions: Vec<i32> = raw_lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    // we'll store all the possible fuel totals for each possible alignment position.
    let mut fuels: Vec<i32> = vec![];

    // we iterate over the possible alignment positions
    for i in 0..positions.len() {
        // summing the fuel required for each crab to move to that position.
        let mut fuel = 0;
        for position in positions.iter() {
            fuel += (position - i as i32).abs();
        }
        fuels.push(fuel);
    }

    // then we just print out the minimum
    let min_fuel = fuels.iter().min().unwrap();

    println!("{:?}", min_fuel);
}