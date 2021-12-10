use super::super::files;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_10/input.txt",
    ));

    // create hashmap from error-able chars to points
    let points_map: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    // a list of the "valid tokens" so we can detect those
    let open_tokens = HashSet::from(['(', '[', '{', '<']);

    // and a map from valid tokens to the expected correct closing tokens
    let valid_pairs = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    // we are gonna store the errors in a vector
    let mut errors = vec![];

    // go through each line
    for line in raw_lines {
        let mut stack = vec![];

        // and through each token
        for c in line.chars() {
            // if it is an "open" token, we put it in a stack
            if open_tokens.contains(&c) {
                stack.push(c);
                continue;
            }

            // if it's a "close" token, we pop the last token from the stack
            // and check if it's a valid pair. if it's not, we push the invalid
            // token to the error list.
            let last_token = stack.pop().unwrap();
            if *valid_pairs.get(&last_token).unwrap() == c {
                continue;
            }

            errors.push(c);
            break;
        }
    }

    // finally, we map the error tokens to points, and sum them all
    let points: u32 = errors.iter().map(|token| points_map.get(token).unwrap()).sum();

    println!("{:?}", points);
}