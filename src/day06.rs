use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day06_test.txt").expect("Failed to read input file");
    let mut p1 = 0;
    let mut p2 = 0;
    let lines: Vec<&str> = input.lines().collect();
    let last_line = lines.last().unwrap();
    let mut ops = vec![];
    for op in last_line.split_whitespace() {
        ops.push(op.to_string());
    }
    let mut grid = vec![];
    // Skip the last line as it contains operations, not numbers
    for line in lines.iter().take(lines.len() - 1) {
        let line = line.trim();
        let mut row = vec![];
        for num in line.split_whitespace() {
            let num: i64 = num.parse().unwrap();
            row.push(num);
        }
        grid.push(row);
    }
    let mut results = vec![];
    for ind in 0..ops.len() {
        let op = &ops[ind];
        if op == "+" {
            // Add numbers in this col
            let sum: i64 = grid.iter().map(|row| row[ind]).sum();
            results.push(sum);
        } else {
            // Multiply numbers in this col
            let product: i64 = grid.iter().map(|row| row[ind]).product();
            results.push(product);
        }
    }
    p1 = results.iter().sum();

    for ind in 0..ops.len() {
        let op = &ops[ind];
        let col_nums: Vec<String> = grid.iter().map(|row| row[ind].to_string()).collect();
        for j in 0..col_nums.len() {
            let num = &col_nums[j];
            println!("{}", num);
        }
        println!("--------------------------------");
    }
    
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}