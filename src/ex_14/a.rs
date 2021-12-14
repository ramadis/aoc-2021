use super::super::files;
use std::collections::HashMap;

fn step(rules: &HashMap<&str, String>, template: &[char]) -> Vec<char> {
    // generates all the pairs
    let mut pairs: Vec<String> = vec![];
    for pair in template.windows(2) {
        pairs.push(pair.iter().collect());
    }
    
    // applies insertions, and generates a list of processed "windows"
    let mut processed_windows = vec![];
    for pair in pairs.iter() {
        match rules.get(pair.as_str()) {
            Some(overwritten) => processed_windows.push(overwritten),
            None => processed_windows.push(pair),
        }
    }
    
    // merges processed windows into a single string
    let mut processed_template = String::new();
    for partial in processed_windows.iter() {
        if processed_template == "" {
            processed_template += &partial[0..1];
        }
        processed_template += &partial[1..];
    }
    
    processed_template.chars().collect()
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_14/input.txt",
    ));

    // parse rules and templates
    let mut rules: HashMap<&str, String> = HashMap::new();
    let mut counts: HashMap<char, u64> = HashMap::new();
    let mut template: Vec<char> = vec![];
    for (idx, line) in raw_lines.iter().enumerate() {
        // the first line is the template
        if idx == 0 {
            template = line.chars().collect();
            continue;
        }

        // for each raw rule, split into a pattern, and an insertion
        if let Some((pattern, insertion)) = line.split_once(" -> ") {
            // generate the insertion string
            let insertion_string = String::from(&pattern[0..1]) + &insertion[0..1] + &pattern[1..2];

            // initialize count map in 0 for every character
            for c in insertion_string.chars() {
                counts.insert(c, 0);
            }

            // insert the mapping from the pattern to the insertion string
            rules.insert(pattern, insertion_string);
        }
    }

    // apply the necessary amount of steps
    for _ in 0..10 {
        template = step(&rules, &template);
    }

    // count the frequency of characters
    for c in template.iter() {
        if let Some(count) = counts.get(&c) {
            counts.insert(*c, count + 1);
        }
    }

    // calculate difference between most and least frequent character
    let (min, max) = (counts.values().min().unwrap(), counts.values().max().unwrap());

    println!("{:?}", max - min);
}