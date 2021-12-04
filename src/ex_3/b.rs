use super::super::files;

fn co2_criteria(nums: &Vec<u32>, idx: u32) -> Vec<u32> {
    let base: u32 = 2;
    let mask = base.pow(4 - idx);
    let common_bit = get_most_common_bit(&nums, idx);

    nums.clone()
        .into_iter()
        .filter(|num| {
            let masked = num & mask;
            if common_bit == 0 {
                masked > 0
            } else {
                masked == 0
            }
        })
        .collect::<Vec<_>>()
}

fn ogr_criteria(nums: &Vec<u32>, idx: u32) -> Vec<u32> {
    let base: u32 = 2;
    let mask = base.pow(4 - idx);
    let common_bit = get_most_common_bit(&nums, idx);

    nums.clone()
        .into_iter()
        .filter(|num| {
            let masked = num & mask;
            if common_bit == 1 {
                masked > 0
            } else {
                masked == 0
            }
        })
        .collect::<Vec<_>>()
}

fn get_most_common_bit(nums: &Vec<u32>, idx: u32) -> u32 {
    let base: u32 = 2;
    let mask = base.pow(4 - idx);
    let count = nums.iter().filter(|&num| num & mask > 0).count();
    if count * 2 >= nums.len() {
        1
    } else {
        0
    }
}

pub fn run() {
    // first we read the raw lines from the input file
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_3/example.txt",
    ));

    // parses the lines into binary numbers
    let mut numbers = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u32>>();

    for i in 0..5 {
        numbers = co2_criteria(&numbers, i);
        println!("{:?}", numbers);
        if numbers.len() == 1 {
            break;
        }
    }
}
