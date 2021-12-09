use super::super::files;
use std::collections::HashMap;

struct SignalSolver {
    input: Vec<String>,
    map: HashMap<u32, String>
}

impl SignalSolver {
    pub fn reverse_lookup(&self, signal: &String) -> Option<u32> {
        for key in self.map.keys() {
            let value = self.map.get(key).unwrap();
            if value == signal {
                return Some(*key);
            }
        }
        None
    }

    pub fn find_0(&mut self) -> &mut Self {
        let signal_0: String = self.input.iter().find(|signal| {
            signal.len() == 6
        }).unwrap().clone();
        self.map.insert(0, signal_0.clone());
        self.input.retain(|signal| *signal != signal_0);
        self
    }

    pub fn find_1(&mut self) -> &mut Self {
        let signal_1: String = self.input.iter().find(|signal| signal.len() == 2).unwrap().clone();
        self.map.insert(1, signal_1.clone());
        self.input.retain(|signal| *signal != signal_1);
        self
    }

    pub fn find_2(&mut self) -> &mut Self {
        self.map.insert(2, self.input[0].clone());
        self
    }

    pub fn find_3(&mut self) -> &mut Self {
        let signal_3: String = self.input.iter().find(|signal| {
            let signal_9 = self.map.get(&9).unwrap();
            signal.chars().all(|c| signal_9.contains(c))
        }).unwrap().clone();
        self.map.insert(3, signal_3.clone());
        self.input.retain(|signal| *signal != signal_3);
        self
    }

    pub fn find_4(&mut self) -> &mut Self {
        let signal_4: String = self.input.iter().find(|signal| signal.len() == 4).unwrap().clone();
        self.map.insert(4, signal_4.clone());
        self.input.retain(|signal| *signal != signal_4);
        self
    }

    pub fn find_5(&mut self) -> &mut Self {
        let signal_5: String = self.input.iter().find(|signal| {
            !self.input.iter().find(|s2| {
                signal.chars().all(|c| s2.contains(c)) && s2.len() == signal.len() + 1
            }).is_none()
        }).unwrap().clone();
        self.map.insert(5, signal_5.clone());
        self.input.retain(|signal| *signal != signal_5);
        self
    }
    pub fn find_6(&mut self) -> &mut Self {
        let signal_6: String = self.input.iter().find(|signal| {
            let signal_5 = self.map.get(&5).unwrap();
            signal.len() == signal_5.len() + 1 && signal_5.chars().all(|c| signal.contains(c))
        }).unwrap().clone();
        self.map.insert(6, signal_6.clone());
        self.input.retain(|signal| *signal != signal_6);
        self
    }

    pub fn find_7(&mut self) -> &mut Self {
        let signal_7: String = self.input.iter().find(|signal| signal.len() == 3).unwrap().clone();
        self.map.insert(7, signal_7.clone());
        self.input.retain(|signal| *signal != signal_7);
        self
    }

    pub fn find_8(&mut self) -> &mut Self {
        let signal_8: String = self.input.iter().find(|signal| signal.len() == 7).unwrap().clone();
        self.map.insert(8, signal_8.clone());
        self.input.retain(|signal| *signal != signal_8);
        self
    }

    pub fn find_9(&mut self) -> &mut Self {
        let signal_9: String = self.input.iter().find(|signal| {
            let signal_4 = self.map.get(&4).unwrap();
            signal.len() == signal_4.len() + 2 && signal_4.chars().all(|c| signal.contains(c))
        }).unwrap().clone();
        self.map.insert(9, signal_9.clone());
        self.input.retain(|signal| *signal != signal_9);
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

    let mut results: Vec<u32> = vec![];
    for (input, output) in notes {
        let mut solver = SignalSolver { input, map: HashMap::new() };

        solver
            .find_1()
            .find_7()
            .find_4()
            .find_8()
            .find_9()
            .find_5()
            .find_6()
            .find_0()
            .find_3()
            .find_2();

        let result_list: Vec<u32> = output.iter().map(|signal| solver.reverse_lookup(signal).unwrap()).collect();
        let result = result_list[0] * 1000 + result_list[1] * 100 + result_list[2] * 10 + result_list[3];
        results.push(result);
    }

    let sum = results.iter().sum::<u32>();
    println!("{:?}", sum);
}