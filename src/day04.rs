use std::fs;

fn count_adjacent_toilet_paper(grid: &Vec<Vec<String>>, current_row: i32, current_col: i32) -> i32 {
    let mut adjacent = 0;
    let directions = [
        (-1, -1), // top-left
        (-1,  0), // top
        (-1,  1), // top-right
        ( 0, -1), // left
        ( 0,  1), // right
        ( 1, -1), // bottom-left
        ( 1,  0), // bottom
        ( 1,  1), // bottom-right
    ];
    
    for (dr, dc) in directions.iter() {
        let new_row = current_row + dr;
        let new_col = current_col + dc;
        
        // Check bounds
        if new_row >= 0 && new_row < grid.len() as i32 &&
           new_col >= 0 && new_col < grid[new_row as usize].len() as i32 {
            if grid[new_row as usize][new_col as usize] == "@" {
                adjacent += 1;
            }
        }
    }
    
    return adjacent;
}

pub fn run() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read input file");
    let mut p1 = 0;
    let mut p2 = 0;
    let mut grid = vec![];
    // We need to check each toilet paper (@) and see if it it has less than 4 adjacent toilet papers (8 adjacent spaces)
    for line in input.lines() {
        let line = line.trim(); // Remove any trailing whitespace
        if line.is_empty() {
            continue; // Skip empty lines
        }
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
            // Now we need to check the 8 immediate adjacent spaces to see if there are less than 4 adjacent toilet papers
            if is_toilet_paper {
                let adjacent = count_adjacent_toilet_paper(&grid, row as i32, col as i32);
                if adjacent < 4 {
                    p1 += 1;
                }
            }
        }
    }
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    let chr = &grid[row][col];
                    let is_toilet_paper = chr == "@";
                    // Now we need to check the 8 immediate adjacent spaces to see if there are less than 4 adjacent toilet papers
                    if is_toilet_paper {
                        let adjacent = count_adjacent_toilet_paper(&grid, row as i32, col as i32);
                        if adjacent < 4 {
                            p2 += 1;
                            // They accessed this roll of toilet paper, we can now remove it
                            grid[row][col] = "x".to_string();
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}