use super::super::files;
use std::collections::HashSet;

type Grid = Vec<Vec<u32>>;
type Position = (usize, usize);

pub fn flash(position: Position, grid: &mut Grid, flash_stack: &mut Vec<Position>, flashed: &mut HashSet<Position>) {
    let (middle_i, middle_j) = position;

    // skip if the octopus in the current position already flashed
    if flashed.contains(&position) {
        return;
    }

    // convert to easily iterate over out-of-bounds indexes
    let middle_i = middle_i as i32;
    let middle_j = middle_j as i32;
    
    // then iterate around the current position:
    // . . . . .
    // . x x x . 
    // . x o x .
    // . x x x .
    // . . . . .
    for i in middle_i - 1..middle_i + 2 {
        for j in middle_j - 1..middle_j + 2 {
            // skip out of bounds indexes
            if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
                continue;
            }
            
            // restore initial types
            let i = i as usize;
            let j = j as usize;
            let middle_i = middle_i as usize;
            let middle_j = middle_j as usize;
            
            // skip the middle (o)
            if middle_i == i && middle_j == j {
                continue;
            }
            
            // otherwise, increase its energy
            grid[i][j] += 1;

            // if energy reached the max, push into the stack of positions to flash.
            if grid[i][j] > 9 {
                flash_stack.push((i, j));
            }
        }
    }

    // register that the current position was already flashed in this step
    flashed.insert(position);
}

// util function to print the grid for debugging purposes
pub fn print_grid(grid: &Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{0: <3}", grid[i][j]);
        }
        println!("");
    }
    println!("");
}

// core function to run the transformations on each step
pub fn step(grid: &mut Grid) -> usize {
    let mut flash_stack:Vec<Position> = vec![];

    // first, iterate to the grid and add 1 to each energy value.
    // if the new value is greater than the energy limit, add it to a
    // stack of positions to "flash".
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            grid[i][j] += 1;
            if grid[i][j] > 9 {
                flash_stack.push((i, j));
            }
        }
    }

    // iterate over the positions we have marked "to flash".
    // and make them flash.
    let mut flashed: HashSet<Position> = HashSet::new();
    while !flash_stack.is_empty() {
        let position = flash_stack.pop().unwrap();
        flash(position, grid, &mut flash_stack, &mut flashed);
    }
    
    // finally, go through the grid again, and transform to 0 every value
    // greater than the lmit (9)
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 9 {
                grid[i][j] = 0;
            }
        }
    }

    // finally return how many times we have flashed on this step
    flashed.len()
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_11/input.txt",
    ));

    // parse octopus grid
    let mut grid: Grid = raw_lines.iter().map(|raw_row| {
        let row: Vec<u32> = raw_row.chars().map(|raw_height| raw_height.to_digit(10).unwrap()).collect();
        row
    }).collect();
    
    // apply each steps as many times as we need
    let mut flashes_count = 0;
    for _ in 0..100 {
        flashes_count += step(&mut grid);
        print_grid(&grid);
    }

    println!("count: {}", flashes_count);
}