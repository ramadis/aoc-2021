use super::super::files;

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_8/input.txt",
    ));

    let notes: Vec<(Vec<&str>, Vec<&str>)> = raw_lines.iter().map(|line| {
        let (raw_input, raw_output) = line.split_once(" | ").unwrap();
        let input_list: Vec<&str> = raw_input.split_whitespace().collect();
        let output_list: Vec<&str> = raw_output.split_whitespace().collect();
        (input_list, output_list)
    }).collect();

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

    let sum: u32 = counts.iter().sum();
    println!("{:?}", sum);
}