use std::fs;

fn check_adjacent_sideways(grid: &Vec<Vec<String>>, current_row: i32, current_col: i32, max_adjacent: i32, check_spots: i32) -> i32 {
    let mut adjacent = 0;
    for i in 1..check_spots {
        let left_col = current_col - i;
        let right_col = current_col + i;
        if left_col < 0 || right_col >= grid[current_row as usize].len() as i32 {
            break;
        }
        let chr_left = &grid[current_row as usize][left_col as usize];
        let chr_right = &grid[current_row as usize][right_col as usize];
        if chr_left == "@" {
            adjacent += 1;
        }
        if chr_right == "@" {
            adjacent += 1;
        }
        if adjacent > max_adjacent {
            return adjacent;
        }
    }
    return adjacent;
}
fn check_adjacent_vertically(grid: &Vec<Vec<String>>, current_row: i32, current_col: i32, max_adjacent: i32, check_spots: i32) -> i32 {
    let mut adjacent = 0;
    for i in 1..check_spots {
        let above_row = current_row - i;
        let below_row = current_row + i;
        if above_row < 0 || below_row >= grid.len() as i32 {
            break;
        }
        let chr_above = &grid[above_row as usize][current_col as usize];
        let chr_below = &grid[below_row as usize][current_col as usize];
        if chr_above == "@" {
            adjacent += 1;
        }
        if chr_below == "@" {
            adjacent += 1;
        }
        if adjacent > max_adjacent {
            return adjacent;
        }
    }
    return adjacent;
}
fn check_adjacent_diagonally(grid: &Vec<Vec<String>>, current_row: i32, current_col: i32, max_adjacent: i32, check_spots: i32) -> i32 {
    let mut adjacent = 0;
    for i in 1..check_spots {
       let right_col = current_col + i;
       let left_col = current_col - i;
       let above_row = current_row - i;
       let below_row = current_row + i;
       if above_row < 0 || below_row >= grid.len() as i32 || left_col < 0 || right_col >= grid[0].len() as i32 {
           break;
       }
       let top_left_char = &grid[above_row as usize][left_col as usize];
       let top_right_char = &grid[above_row as usize][right_col as usize];
       let bottom_left_char = &grid[below_row as usize][left_col as usize];
       let bottom_right_char = &grid[below_row as usize][right_col as usize];
       if top_left_char == "@" {
        adjacent += 1;
       }
       if top_right_char == "@" {
        adjacent += 1;
       }
       if bottom_left_char == "@" {
        adjacent += 1;
       }
       if bottom_right_char == "@" {
        adjacent += 1;
       }
       if adjacent > max_adjacent {
        return adjacent;
       }
    }
    return adjacent;
}

pub fn run() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read input file");
    let mut p1 = 0;
    let p2 = 0;
    let mut grid = vec![];
    // We need to check each toilet paper (@) and see if it it has less than 4 adjacent toilet papers (8 adjacent spaces)
    for line in input.lines() {
        let mut sub_grid = vec![];
        for chr in line.chars() {
            sub_grid.push(chr.to_string());
        }
        grid.push(sub_grid);
    }
    // We now have a 2D array of the toilet paper characters
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let chr = &grid[row][col];
            let is_toilet_paper = chr == "@";
            // Now we need to traverse above, below, left, right, and diagonally to see if there are less than 4 adjacent in 8 adjacent spaces
            if is_toilet_paper {
                let mut adjacent: i32 = 0;
                adjacent += check_adjacent_sideways(&grid, row as i32, col as i32, 4, 8);
                adjacent += check_adjacent_vertically(&grid, row as i32, col as i32, 4, 8);
                adjacent += check_adjacent_diagonally(&grid, row as i32, col as i32, 4, 8);
                if adjacent < 4 {
                    p1 += 1;
                }
            }
        }
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}