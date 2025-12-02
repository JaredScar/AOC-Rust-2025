use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read input file");
    let mut dial_pos = 50;
    let mut password = 0;
    let mut p2 = 0;
    let mut rots = vec![];
    for line in input.lines() {
        let (dir, num_str) = line.split_at(1);
        let mut movement: i32 = num_str.parse().unwrap();
        if dir == "L" {
            movement = -movement;
        }
        rots.push(movement);
    }
    for rot in rots {
        let abs_rot = rot.abs();
        for _ in 0..abs_rot {
            if rot < 0 {
                // Left
                dial_pos = (dial_pos - 1 + 100) % 100;
            } else {
                // Right
                dial_pos = (dial_pos + 1) % 100;
            }
            if dial_pos == 0 {
                p2 += 1;
            }
        }
        if dial_pos == 0 {
            password += 1;
        }
    }
    println!("Password: {}", password);
    println!("Password 2: {}", p2);
}