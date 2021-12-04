use super::super::files;

pub fn run() {
    // first we read the raw lines from the input file
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_3/input.txt",
    ));

    // store the size of the binary string
    let size = lines[0].len();

    // parses the lines into binary numbers
    let numbers = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap());

    // the way the algorithm works is:
    // 1) create an array filled with 0s [0, 0, ...] with the same length of input numbers.
    // 2) iteratively, mask each number with 10000, 01000, 00100, 000010, 000001, ...
    // 3) if the result is greater than 0, that means there was a 1 in the original number
    // 4) if that's the case, increase by one the count in the same index.
    // this way, we end with an array, where each element is the # of 1s for the given index
    let base: u32 = 2;
    let mut count = vec![0; size];
    for number in numbers.clone() {
        for i in 0..size {
            let num_mask = base.pow((size - 1 - i) as u32);
            let masked = num_mask & number;
            if masked > 0 {
                count[i as usize] += 1;
            }
        }
    }

    // then we transform that "count" array into a binary string.
    // if twice the count is greater the # of numbers (ie. there are more 0s than 1s), we put a 1.
    // otherwise we put a 0.
    let str_gamma_rate = count
        .iter()
        .map(|n| {
            if numbers.len() < n * 2 {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec<&str>>()
        .join("");

    // by the end, we parse the binary string into a number. that's the "gamma rate".
    // the "epsilon rate" is just the bitwise negation of the "gamma rate".
    // the only catch is we have to mask the gamma rate because otherwise we are
    // putting 1s in the most significative bits of the epsilon_rate variable.
    let gamma_rate = u32::from_str_radix(&str_gamma_rate, 2).unwrap();
    let epsilon_rate = !gamma_rate & (base.pow(size as u32) - 1);
    let result = gamma_rate * epsilon_rate;

    println!("gamma_rate: {}, epsilon_rate: {}, result: {}", gamma_rate, epsilon_rate, result);
}
