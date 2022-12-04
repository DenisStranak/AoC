use std::fs;
use std::ops::Range;

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let elves = line.split(',').collect::<Vec<_>>();
        let elf1_range = get_range(elves[0]);
        let elf2_range = get_range(elves[1]);

        if fully_contains(&elf1_range, &elf2_range) {
            part1 += 1;
        }
        if is_overlapping(&elf1_range, &elf2_range) {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_range(elf: &str) -> Range<i32> {
    let input = elf.split('-').collect::<Vec<_>>();
    let range = input[0].parse::<i32>().unwrap()..input[1].parse::<i32>().unwrap();

    return range;
}

fn fully_contains(range1: &Range<i32>, range2: &Range<i32>) -> bool {
    if range1.start >= range2.start && range1.end <= range2.end
        || range2.start >= range1.start && range2.end <= range1.end
    {
        return true;
    }

    return false;
}

fn is_overlapping(range1: &Range<i32>, range2: &Range<i32>) -> bool {
    return range1.start <= range2.end && range2.start <= range1.end;
}
