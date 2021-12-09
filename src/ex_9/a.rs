use super::super::files;

pub fn get_surrounding_heights(map: &Vec<Vec<u32>>, i: usize, j: usize) -> Vec<Option<u32>> {
    // not much magic here, just bound checks.
    let left = if i as i32 - 1 >= 0 { Some(map[i - 1 as usize][j]) } else { None };
    let right = if i + 1 < map.len() { Some(map[i + 1 as usize][j]) } else { None };
    let top = if j as i32 - 1 >= 0 { Some(map[i as usize][j - 1]) } else { None };
    let bottom = if j + 1 < map[0].len() { Some(map[i as usize][j + 1]) } else { None };
    let around = vec![left, top, right, bottom];
    return around;
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_9/input.txt",
    ));

    // parse heightmap
    let map: Vec<Vec<u32>> = raw_lines.iter().map(|raw_row| {
        let row: Vec<u32> = raw_row.chars().map(|raw_height| raw_height.to_digit(10).unwrap()).collect();
        row
    }).collect();

    // we'll be storing the low points into an vector.
    let mut low_points = vec![];

    // iterate through the map elements.
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // for each element, get the list of surrounding cells.
            let cells = get_surrounding_heights(&map, i, j);

            // if all the cells around are bigger, this is a low point
            let is_low = cells.iter().all(|cell| {
                match cell {
                    Some(c) => *c > map[i][j],
                    None => true
                }
            });

            // if so, store in the vector
            if is_low {
                low_points.push(map[i][j]);
            }
        }
    }

    // transform low points into risks, and sum
    let risk_sum = low_points.iter().map(|&low_point| low_point + 1).sum::<u32>();
    println!("{:?}", risk_sum);
}