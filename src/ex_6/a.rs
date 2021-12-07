use super::super::files;

const N: u32 = 80;

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_6/input.txt",
    ));

    let mut values: Vec<u8> = raw_lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    let mut new_values: Vec<u8> = vec![];

    // for each day
    for _ in 1..(N+1) {
        for value in values.iter() {
            // if the value is 0, reset the fish's days, and insert a new fish with value 8
            if *value == 0 {
                new_values.push(6);
                new_values.push(8);
            } else {
                // otherwise, just reduce the number
                new_values.push(value - 1);
            }
        }
        values = new_values;
        new_values = vec![];
    }

    let count = values.iter().count();
    println!("count: {}", count);

}