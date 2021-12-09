use super::super::files;
use std::collections::HashMap;

struct Input {
    list: Vec<String>,
}

impl Input {
    pub fn find_1(&mut self, map: &mut HashMap<u32, String>) -> &Self {
        let signal_1: String = self.list.iter().find(|signal| signal.len() == 2).unwrap().clone();
        map.insert(1, signal_1.clone());
        self.list.retain(|signal| *signal != signal_1);
        self
    }
}

pub fn sort_string(s: &str) -> String {
    let mut char_arr: Vec<char> = s.chars().collect();
    char_arr.sort_unstable();
    let sorted_s: String = char_arr.into_iter().collect();
    sorted_s
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_8/input.txt",
    ));

    // parse input, and sort each individual word to make things easier to debug
    let notes: Vec<(Vec<String>, Vec<String>)> = raw_lines.iter().map(|line| {
        let (raw_input, raw_output) = line.split_once(" | ").unwrap();
        let input_list: Vec<String> = raw_input.split_whitespace().map(|signal| sort_string(signal)).collect();
        let output_list: Vec<String> = raw_output.split_whitespace().map(|signal| sort_string(signal)).collect();
        (input_list, output_list)
    }).collect();

    // here is my line of thought. as per the last exercise we know:
    // "ab" -> 1
    // "abd" -> 7
    // "abef" -> 4
    // "abcdefg" -> 8
    // meaning, from 1 and 7 I always can obtain the mapping for "a". in this case:
    // d -> a
    // this also means that from the number 8 we can obtain no information, since it
    // uses all possible wires and order has no meaning.
    // ...
    // look at number 9. it is basically 4 with two extra wires. this means that we
    // can deduce which one is number 9 by looking which signal string is a superset
    // from the 4 signal string (and is not 8 lol)
    // in the example case:
    // "abcdef" includes "abef" and has two more lines, so this is clearly 9.
    // "abcdef" -> 9
    // ...
    // we already know the mapping for "a" ("d"), so that means we know can figure
    // the mapping for "g":
    // abcdef - abef = cd, and d -> a => c -> g
    // ...
    // following analogous logic, 6 is just 5 with an extra line. and it's the only number
    // for which this happens (that is not 1, 7, 4, 8, *or 9*).
    // this is a bit more complicated because we don't know how any of 5 or 6 looks like.
    // however, we can look into which of the remaining signals has one more line than another one
    // whichever is shorter is 5. this leaves us with:
    // "bcdef" -> 5
    // "bcdefg" -> 6
    // these also defines for us the mapping for "e" (the signals' diff). in this case:
    // g -> e.
    // ...
    // we are missing 0, 2, 3. From these, 0 is the longest. thus:
    // "abcdeg" -> 0
    // look at number 9, Is basically 3 with an extra line, so we look which of the remaining 2
    // is a subset of 9's signal.
    // and since "abcdf" c "abcdef":
    // "abcdf" -> 3
    // and of course:
    // "acdfg" -> 2.
    // ...
    // finally the output is:
    // bcdef abcdf bcdef abcdf
    // 5 3 5 3
    // now we only have to write this logic into code...

    let mut number_map:HashMap<u32, String> = HashMap::new();
    let mut signal_map:HashMap<String, u32> = HashMap::new();
    let mut results: Vec<u32> = vec![];
    for (mut input, output) in notes {
        // let mut input = Input {
        //     list: input
        // };
        // input.find_1(&mutnumber_map);

        // fetch 1
        let signal_1: String = input.iter().find(|signal| signal.len() == 2).unwrap().clone();
        number_map.insert(1, signal_1.clone());
        signal_map.insert(signal_1.clone(), 1);
        input.retain(|signal| *signal != signal_1);

        // fetch 7
        let signal_7: String = input.iter().find(|signal| signal.len() == 3).unwrap().clone();
        number_map.insert(7, signal_7.clone());
        signal_map.insert(signal_7.clone(), 7);
        input.retain(|signal| *signal != signal_7);

        // fetch 4
        let signal_4: String = input.iter().find(|signal| signal.len() == 4).unwrap().clone();
        number_map.insert(4, signal_4.clone());
        signal_map.insert(signal_4.clone(), 4);
        input.retain(|signal| *signal != signal_4);

        // fetch 8
        let signal_8: String = input.iter().find(|signal| signal.len() == 7).unwrap().clone();
        number_map.insert(8, signal_8.clone());
        signal_map.insert(signal_8.clone(), 8);
        input.retain(|signal| *signal != signal_8);

        // fetch 9
        let signal_9: String = input.iter().find(|signal| {
            let signal_4 = number_map.get(&4).unwrap();
            signal.len() == signal_4.len() + 2 && signal_4.chars().all(|c| signal.contains(c))
        }).unwrap().clone();
        number_map.insert(9, signal_9.clone());
        signal_map.insert(signal_9.clone(), 9);
        input.retain(|signal| *signal != signal_9);

        // fetch 5
        let signal_5: String = input.iter().find(|signal| {
            !input.iter().find(|s2| {
                signal.chars().all(|c| s2.contains(c)) && s2.len() == signal.len() + 1
            }).is_none()
        }).unwrap().clone();
        number_map.insert(5, signal_5.clone());
        signal_map.insert(signal_5.clone(), 5);
        input.retain(|signal| *signal != signal_5);

        // fetch 6
        let signal_6: String = input.iter().find(|signal| {
            let signal_5 = number_map.get(&5).unwrap();
            signal.len() == signal_5.len() + 1 && signal_5.chars().all(|c| signal.contains(c))
        }).unwrap().clone();
        number_map.insert(6, signal_6.clone());
        signal_map.insert(signal_6.clone(), 6);
        input.retain(|signal| *signal != signal_6);

        // fetch 0
        let signal_0: String = input.iter().find(|signal| {
            signal.len() == 6
        }).unwrap().clone();
        number_map.insert(0, signal_0.clone());
        signal_map.insert(signal_0.clone(), 0);
        input.retain(|signal| *signal != signal_0);

        // fetch 3
        let signal_3: String = input.iter().find(|signal| {
            let signal_9 = number_map.get(&9).unwrap();
            signal.chars().all(|c| signal_9.contains(c))
        }).unwrap().clone();
        number_map.insert(3, signal_3.clone());
        signal_map.insert(signal_3.clone(), 3);
        input.retain(|signal| *signal != signal_3);

        // fetch 2
        number_map.insert(2, input[0].clone());
        signal_map.insert(input[0].clone(), 2);

        let result: Vec<u32> = output.iter().map(|signal| signal_map.get(signal).unwrap().clone()).collect();
        results.push(result[0] * 1000 + result[1] * 100 + result[2] * 10 + result[3]);
    }

    let sum = results.iter().sum::<u32>();
    println!("{:?}", sum);
}