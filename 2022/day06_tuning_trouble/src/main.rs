use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    println!("Part 1: {}", find_end_of_sequence(&input, 4));
    println!("Part 2: {}", find_end_of_sequence(&input, 14));
}

fn find_end_of_sequence(input: &str, sequence_size: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    let windows = chars.windows(sequence_size);

    for (i, window) in windows.enumerate() {
        let mut sequence: HashSet<char> = HashSet::new();

        sequence.extend(window);

        if sequence.len() == sequence_size {
            return i + sequence_size;
        }
    }

    return 0;
}
