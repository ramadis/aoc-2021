use super::super::files;
use std::str::FromStr;
use std::string::ParseError;

const GRID_SIZE: u32 = 5;

#[derive(Debug)]
struct Board {
    won: bool,
    rows: Vec<Vec<u32>>,
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<u32>> = vec![];
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
            let row = chunk.iter().map(|x| x.parse().unwrap()).collect();
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

    let boards: Vec<Board> = lines
        .join("\n")
        .split("\n\n")
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("{:?}", boards);
}