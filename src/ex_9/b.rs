use super::super::files;

pub fn find_area_idxs(map: &Vec<Vec<u32>>) -> Option<(i32, i32)> {
    // since we are filling with 9s the basins as we go through them,
    // we need a mechanism to find the next available basin. to that end,
    // we iterate through the heightmap searching for a non-9 value. if we find
    // one, we return the indexes. Otherwise, we return None.
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != 9 {
                return Some((i as i32, j as i32));
            }
        }
    }

    None
}

pub fn find_basin_size(map: &mut Vec<Vec<u32>>, i: i32, j: i32) -> u32 {
    // if the indexes are out of bound, or we reach a basin border (a 9),
    // we are not in a basin
    if i < 0 || i >= map.len() as i32 {
        return 0;
    } else if j < 0 || j >= map[0].len() as i32 {
        return 0;
    } else if map[i as usize][j as usize] == 9 {
        return 0;
    }

    // put a 9 (basin border) in the current position
    map[i as usize][j as usize] = 9;

    // sum 1 (the current position, which is within a basin), and look around
    // for other positions within a basin. sum all those sizes.
    let sum = 1 +
        find_basin_size(map, i, j + 1) +
        find_basin_size(map, i + 1, j) +
        find_basin_size(map, i, j - 1) +
        find_basin_size(map, i - 1, j);

    sum
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_9/input.txt",
    ));

    // parse heightmap
    let mut map: Vec<Vec<u32>> = raw_lines.iter().map(|raw_row| {
        let row: Vec<u32> = raw_row.chars().map(|raw_height| raw_height.to_digit(10).unwrap()).collect();
        row
    }).collect();

    // we are gona store the basin sizes in a vector
    let mut basin_sizes = vec![];

    // iterate through possible basins, if any
    while let Some((i, j)) = find_area_idxs(&map) {
        // when one is found, store the basin size.
        basin_sizes.push(find_basin_size(&mut map, i, j));
    }

    // then sort it in desc. order
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    // and multiply the top 3 results
    println!("{:?}", basin_sizes[0..3].iter().product::<u32>());
}