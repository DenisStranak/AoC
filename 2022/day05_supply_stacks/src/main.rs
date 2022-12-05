use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let (crates, moves) = input.split_once("\n\n").unwrap();

    let crate_stacks = parse_crates(crates);
    let moves = parse_moves(moves);

    let part1_crate_stacks = apply_moves(&crate_stacks, &moves, true);
    let part2_crate_stacks = apply_moves(&crate_stacks, &moves, false);

    println!("Part 1: {:?}", get_top_crates(part1_crate_stacks));
    println!("Part 2: {:?}", get_top_crates(part2_crate_stacks));
}

fn parse_crates(input: &str) -> Vec<VecDeque<char>> {
    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;

    let mut crates = vec![VecDeque::new(); stack_count];
    for line in input.lines() {
        for (i, char) in line.chars().enumerate() {
            if char.is_alphabetic() {
                crates[i / 4].push_front(char);
            }
        }
    }

    return crates;
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let mut moves: Vec<(usize, usize, usize)> = vec![];

    for line in input.lines() {
        let instructions: Vec<&str> = line
            .split_whitespace()
            .filter(|&x| x.parse::<usize>().is_ok())
            .collect();

        let (amount, from, to) = (
            instructions[0].parse::<usize>().unwrap(),
            instructions[1].parse::<usize>().unwrap(),
            instructions[2].parse::<usize>().unwrap(),
        );
        moves.push((amount, from, to));
    }

    return moves;
}

fn apply_moves(
    crate_stacks: &Vec<VecDeque<char>>,
    moves: &Vec<(usize, usize, usize)>,
    reverse: bool,
) -> Vec<VecDeque<char>> {
    let mut crate_stacks = crate_stacks.clone();

    for (amount, from, to) in moves {
        let drain_index = crate_stacks[from - 1].len() - amount;
        let crates = crate_stacks[from - 1]
            .drain(drain_index..)
            .collect::<Vec<_>>();

        if reverse {
            crate_stacks[to - 1].extend(crates.into_iter().rev());
        } else {
            crate_stacks[to - 1].extend(crates);
        }
    }

    return crate_stacks;
}

fn get_top_crates(crates: Vec<VecDeque<char>>) -> String {
    let mut top_crates: String = String::new();

    for mut current_crate in crates {
        top_crates.push(current_crate.pop_back().unwrap());
    }

    return top_crates;
}
