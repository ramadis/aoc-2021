use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_lines(filename: String) -> Vec<String> {
    let mut vec = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(string) = line {
                vec.push(string);
            }
        }
    }

    return vec;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}