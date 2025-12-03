use std::fs;

fn find_first_largest_digit(line: &str) -> (i32, i32) {
    let mut largest_digit = 0;
    let mut ind = 0;
    let mut ret_ind = 0;
    for digit in line.chars() {
        let digit_num: i32 = digit.to_digit(10).unwrap() as i32;
        if digit_num > largest_digit && line.chars().nth(ind as usize + 1) != None {
            largest_digit = digit_num;
            ret_ind = ind;
        }
        ind += 1;
    }
    return (largest_digit, ret_ind);
}
fn find_second_largest_digit(line: &str, start_index: i32) -> i32 {
    let mut largest_digit = 0;
    for digit in line[(start_index + 1) as usize..].chars() {
        let digit_num: i32 = digit.to_digit(10).unwrap() as i32;
        if digit_num > largest_digit {
            largest_digit = digit_num;
        }
    }
    return largest_digit;
}

pub fn run() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read input file");
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        // Each line of digits in the input is a single bank of batteries
        // Within each bank we need to turn on exactly 2 batteries.
        let first_largest_digit_and_index = find_first_largest_digit(line);
        let first_largest_digit = first_largest_digit_and_index.0;
        let first_largest_digit_index = first_largest_digit_and_index.1;
        let second_largest_digit = find_second_largest_digit(line, first_largest_digit_index);
        let num_str = first_largest_digit.to_string() + &second_largest_digit.to_string();
        let num: i32 = num_str.parse().unwrap();
        println!("Num: {}", num);
        p1 += num;
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}