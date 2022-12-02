use std::fs;

static ROCK: u32 = 1;
static PAPER: u32 = 2;
static SCISSORS: u32 = 3;

static WIN: u32 = 6;
static DRAW: u32 = 3;
static LOSS: u32 = 0;

static POINT_MATRIX: [[u32; 3]; 3] = [
    [ROCK + DRAW, PAPER + WIN, SCISSORS + LOSS],
    [ROCK + LOSS, PAPER + DRAW, SCISSORS + WIN],
    [ROCK + WIN, PAPER + LOSS, SCISSORS + DRAW],
];

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let mut part1_points: u32 = 0;
    let mut part2_points: u32 = 0;
    for line in input.lines() {
        let round = line.split(" ").collect::<Vec<&str>>();
        let (opponent_move, my_move) = parse_round(&round);

        part1_points += POINT_MATRIX[opponent_move][my_move];
        part2_points += POINT_MATRIX[opponent_move][(opponent_move + my_move + 2) % 3];
    }

    println!("Part 1: {}", part1_points);
    println!("Part 2: {}", part2_points);
}

fn parse_round(round: &Vec<&str>) -> (usize, usize) {
    let opponent_move = round[0].chars().next().unwrap() as usize - 65;
    let my_move = round[1].chars().next().unwrap() as usize - 88;

    return (opponent_move, my_move);
}
