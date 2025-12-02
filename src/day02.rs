use std::fs;

fn count_digits(num: i64) -> usize {
    return num.abs().to_string().len();
}

fn is_invalid_number(num: i64) -> bool {
    let digitCount = count_digits(num) / 2;
    return num.to_string()[0..digitCount].parse::<i64>().unwrap() == num.to_string()[digitCount..].parse::<i64>().unwrap();
}

pub fn run() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read input file");
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let ranges = line.split(",");
        for range in ranges {
            let mut parts = range.split("-");
            let start: i64 = parts.next().unwrap().parse().unwrap();
            let end: i64 = parts.next().unwrap().parse().unwrap();
            for counter in start..=end {
                let len = count_digits(counter);
                if len % 2 == 0 {
                    // Can have a valid invalid number, check if it does
                    if is_invalid_number(counter) {
                        p1 += counter;
                    }
                }
                for chunk_size in 1..=len / 2 {
                    if len % chunk_size != 0 {
                        continue; // We only want to continue if the chunks are of equal length for comparison
                    }
                    let mut hasEqualChunks = true;
                    let mut i = chunk_size;
                    let first_chunk = counter.to_string()[0..chunk_size].parse::<i64>().unwrap();
                    // Loop through valid chunks and check if they are all equal
                    while i < len {
                        let next_chunk = counter.to_string()[i..i+chunk_size].parse::<i64>().unwrap();
                        if first_chunk != next_chunk {
                            hasEqualChunks = false;
                            break;
                        }
                        i += chunk_size;
                    }
                    // Chunks are all equal, add to part 2, then go to next number for validation
                    if hasEqualChunks {
                        p2 += counter;
                        break;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}