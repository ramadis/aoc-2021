use super::super::files;
use std::collections::HashMap;

const N: u64 = 256;
const BORDER: u64 = N + 9 + 1;

// This is the recursive function we use to solve the recurrence equation.
fn calculate (prev: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    // The base case happens when a value is bigger than a border condition.
    // The border condition was derived from the aoc example.
    // It's basically N (the number of days) + 9 (new fishes on the last day) + 1 (edge condition)
    if prev >= BORDER {
        return 0;
    }

    // We check if the value is already cached. If so, we return that value
    if cache.contains_key(&prev) {
        let cached = *cache.get(&prev).unwrap();
        return cached;
    }

    // Finally, we solve the equation by iterating over X.
    // Once we find a valid value, we calculate the children of that value
    // recursively.
    let mut count = 0;
    for x in 0..BORDER {
        let v = prev + 7 * x + 9;
        if v < BORDER {
            count += 1 + calculate(v, cache);
        }
    }

    cache.insert(prev, count);
    count
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_6/input.txt",
    ));

    // parse values and transform to offsets. each number will represent how many
    // times we have to move the column to end up with a 0 in the first row
    // (i'm speaking about the rendered example in the aoc page)
    let values: Vec<u64> = raw_lines[0].split(',').map(|x| x.parse().unwrap()).map(|x: u64| x+1).collect();

    // It took me hours, but I realized that we can express the growth of each one
    // of the fishes as a recurrence equations. Take the first value, for example (4):
    // The growth can be expressed as follows:
    // To = 4 -> The initial value is a solution we should count
    // T1 = 4 + 7x + 9; -> Every 7 iterations, we add a new fish (which starts with an initial offset of 9 + whatever the accumulated offset is)
    // T2 = (4 + 7x + 9) + 7y + 9; -> Note that both variables are different (x, and y)
    // ...
    // Tn = Tn-1 + 7n + 9; -> Finally, this is the general form.
    
    // We iterate through the initial values, since we have to count all recurrence equations:
    // To = 4; To = 3; To = 5; ...
    // We have to solve the equation for each of those, and then sum the results.
    // We also will use a cache to store pre-calculated values, this is the difference
    // between the script running in 0.5s or never ending.
    let mut cache:HashMap<u64, u64> = HashMap::new();
    let mut count: u64 = values.len() as u64;
    for value in values.iter() {
        count += calculate(*value, &mut cache);
    }

    println!("count: {}", count);
}