use super::super::files;
use std::str::FromStr;
use std::string::ParseError;

const GRID_SIZE: u32 = 5;

#[derive(Debug, Copy, Clone)]
struct Number {
    value: u32,
    marked: bool,
}

impl Number {
    fn mark(&mut self) {
        self.marked = true;
    }
}

type Matrix = Vec<Vec<Number>>;

#[derive(Debug, Clone)]
struct Board {
    won: bool,
    rows: Matrix,
}

impl Board {
    fn mark_number(&mut self, num: u32) {
        for row in self.rows.iter_mut() {
            for number in row.iter_mut() {
                if (number.value == num) {
                    number.mark();
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u32 {
        self.rows.iter().flatten().filter(|n| !n.marked).map(|n| n.value).sum()
    }

    fn has_won(&self) -> bool {
        // check rows
        for row in self.rows.iter() {
            if row.iter().all(|n| n.marked) {
                return true;
            }
        }

        // check columns

        for i in 0..GRID_SIZE {
            let mut all_marked = true;

            for row in self.rows.iter() {
                if !row[i as usize].marked {
                    all_marked = false;
                }
            }

            if (all_marked) {
                return true;
            }
        }

        // otherwise the game keeps going on
        false
    }
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<Number>> = vec![];
        let chunks = raw
            .split_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
            .join(",")
            .split(",")
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
            .chunks_exact(GRID_SIZE as usize)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<String>>>();
            
        for chunk in chunks {
            let row = chunk.iter().map(|x| {
                Number {
                    value: x.parse().unwrap(),
                    marked: false,
                }
            }).collect();
            rows.push(row);
        }

        Ok(Board { won: false, rows })
    }
}

pub fn run() {
    // first we read the raw lines from the input file
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_4/example.txt",
    ));

    // parse the drawn numbers
    let (raw_drawn_numbers, lines) = lines.split_first().unwrap();
    let drawn_numbers: Vec<u32> = raw_drawn_numbers.split(',').map(|x| x.parse().unwrap()).collect();

    println!("drawn_numbers: {:?}", drawn_numbers);

    let mut boards: Vec<Board> = lines
        .join("\n")
        .split("\n\n")
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if (board.has_won()) {
                let sum = board.sum_unmarked();
                println!("Game finished! The sum is {}, the number {}, and the result {}", sum, number, sum * number);
                return;
            }
        }
    }

    println!("{:?}", boards);
}