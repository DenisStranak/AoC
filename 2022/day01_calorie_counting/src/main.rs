use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    println!("Part 1: {}", get_top_calories(input.lines(), 1));
    println!("Part 2: {}", get_top_calories(input.lines(), 3));
}

fn get_top_calories(calories: std::str::Lines<'_>, count: i32) -> i32 {
    let mut best: BinaryHeap<i32> = BinaryHeap::new();
    let mut current: i32 = 0;

    for line in calories {
        if !line.is_empty() {
            current += line.parse::<i32>().unwrap();
        } else {
            best.push(current);
            current = 0;
        }
    }

    let mut sum = 0;
    for _ in 0..count {
        sum += best.pop().unwrap();
    }

    return sum;
}
