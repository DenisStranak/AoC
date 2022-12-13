use std::str::FromStr;
use std::{collections::VecDeque, fs};

#[derive(Default, Clone)]
enum Operation {
    #[default]
    Square,
    Add(u64),
    Mul(u64),
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        return match self {
            Operation::Square => item * item,
            Operation::Add(n) => item + n,
            Operation::Mul(n) => item * n,
        };
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, value) = s
            .strip_prefix("new = old ")
            .unwrap()
            .split_once(" ")
            .unwrap();
        if operation == "*" && value == "old" {
            Ok(Operation::Square)
        } else if operation == "+" {
            Ok(Operation::Add(value.parse().unwrap()))
        } else {
            Ok(Operation::Mul(value.parse().unwrap()))
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

impl Monkey {
    fn turn(&mut self, divisor: u64) -> Vec<(usize, u64)> {
        let mut actions: Vec<(usize, u64)> = Vec::new();
        while self.items.len() > 0 {
            self.inspection_count += 1;
            let mut item = self.items.pop_front().unwrap();

            item = self.operation.apply(item);

            if divisor > 1 {
                item %= divisor;
            } else {
                item /= 3;
            }

            if item % self.test == 0 {
                actions.push((self.true_target, item));
            } else {
                actions.push((self.false_target, item));
            }
        }

        return actions;
    }
}

fn main() {
    let input = fs::read_to_string("src\\input.txt").expect("Unable to read file");

    let mut part1_monkeys: Vec<Monkey> = parse_monkeys(&input);
    let mut part2_monkeys: Vec<Monkey> = part1_monkeys.clone();

    for _ in 0..20 {
        simulate_round(&mut part1_monkeys, 0);
    }

    let common_divisor = part1_monkeys.iter().map(|m| m.test).fold(1, |a, b| a * b);
    for _ in 0..10000 {
        simulate_round(&mut part2_monkeys, common_divisor);
    }

    println!("Part 1: {}", calculate_monkey_business(&mut part1_monkeys));
    println!("Part 2: {}", calculate_monkey_business(&mut part2_monkeys));
}

fn simulate_round(monkeys: &mut Vec<Monkey>, divisor: u64) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let actions = monkey.turn(divisor);
        for action in actions {
            monkeys[action.0].items.push_back(action.1);
        }
    }
}

fn calculate_monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));

    return monkeys[0].inspection_count * monkeys[1].inspection_count;
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in input.split("\n\n").collect::<Vec<&str>>().into_iter() {
        monkeys.push(parse_monkey(monkey));
    }

    return monkeys;
}

fn parse_monkey(monkey: &str) -> Monkey {
    let lines: Vec<&str> = monkey
        .lines()
        .map(|l| l.split(": ").last().unwrap())
        .collect();

    let items = lines[1]
        .split(", ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>();

    let operation = Operation::from_str(lines[2]).unwrap();

    let test = lines[3].rsplit_once(" ").unwrap().1.parse::<u64>().unwrap();

    let true_target = lines[4]
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let false_target = lines[5]
        .rsplit_once(" ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();

    let monkey = Monkey {
        items,
        operation,
        test,
        true_target,
        false_target,
        inspection_count: 0,
    };

    return monkey;
}
