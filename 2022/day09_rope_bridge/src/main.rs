use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let mut head = (0, 0);
    let mut tail = [(0, 0); 9];

    let mut part1: HashSet<(i32, i32)> = HashSet::new();
    let mut part2: HashSet<(i32, i32)> = HashSet::new();
    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<i32>().unwrap();

        for _ in 0..steps {
            let (dx, dy) = match direction {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => (0, 0),
            };
            head = (head.0 + dx, head.1 + dy);

            tail[0] = move_tail(tail[0], head);
            for i in 1..9 {
                tail[i] = move_tail(tail[i], tail[i - 1]);
            }

            part1.insert(tail[0]);
            part2.insert(tail[8]);
        }
    }

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());
}

fn move_tail(source: (i32, i32), target: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (target.0 - source.0, target.1 - source.1);

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return source;
    }

    return (source.0 + dx.signum(), source.1 + dy.signum());
}
