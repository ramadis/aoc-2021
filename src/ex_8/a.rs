use super::super::files;

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_8/input.txt",
    ));

    // parse input
    let notes: Vec<(Vec<&str>, Vec<&str>)> = raw_lines.iter().map(|line| {
        let (raw_input, raw_output) = line.split_once(" | ").unwrap();
        let input_list: Vec<&str> = raw_input.split_whitespace().collect();
        let output_list: Vec<&str> = raw_output.split_whitespace().collect();
        (input_list, output_list)
    }).collect();

    // we will keep track of the count for a specific number in this fixed-length array.
    // this part seems pretty easy, because the numbers we are asked to count, are the
    // ones that are identifiable by their length alone. so only by looking at the length
    // we can deduce how many 1, 7, 4, and 8s there are in the output.
    let mut counts: [u32; 10] = [0; 10];
    for (_, output) in notes {
        for signal in output {
            let length = signal.len();
            if length == 2 {
                counts[1] += 1;
            } else if length == 3 {
                counts[7] += 1;
            } else if length == 4 {
                counts[4] += 1;
            } else if length == 7 {
                counts[8] += 1;
            }
        }
    }

    // finally, we sum the counts, as asked by the exercise
    let sum: u32 = counts.iter().sum();
    println!("{:?}", sum);
}