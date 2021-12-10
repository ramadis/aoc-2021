use super::super::files;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_10/input.txt",
    ));

    // create hashmap from error-able chars to points
    let points_map: HashMap<char, u64> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4)
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

    // we are gonna store the scores in a vector
    let mut scores: Vec<u64> = vec![];

    // go through each line
    'lines: for line in raw_lines {
        let mut stack = vec![];

        // and through each token
        for c in line.chars() {
            // if it is an "open" token, we put it in a stack
            if open_tokens.contains(&c) {
                stack.push(c);
                continue;
            }

            // ignore corrupted lines
            let last_token = stack.pop().unwrap();
            if *valid_pairs.get(&last_token).unwrap() != c {
                continue 'lines; 
            }
        }

        // if we reached here, the stack is valid
        // if the stack is empty, the line is complete, so we can cut early.
        if stack.len() == 0 {
            continue;
        }

        // otherwise, the line is incomplete.
        // first reverse the "stack" so we can iterate over it as if it were an
        // actual stack.
        stack.reverse();

        // then, map through each "open" token, and convert it to its valid "close" token.
        // finally, reduce the list of "close" tokens, following the operation
        // defined by the exercise
        let score = stack
            .iter()
            .map(|token| valid_pairs.get(&token).unwrap())
            .fold(0, |points, token| {
                points * 5 + points_map.get(&token).unwrap()
            });
        scores.push(score);
    }

    // sort the scores
    scores.sort_unstable();

    // and get the middle one
    let score = scores[(scores.len() / 2) as usize];
    println!("{:?}", score);

}