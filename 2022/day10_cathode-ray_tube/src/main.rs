use std::fs;

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let mut x = 1;
    let mut signals = Vec::new();
    for line in input.lines() {
        if line == "noop" {
            signals.push(x);
        } else {
            let value = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();

            signals.push(x);
            signals.push(x);

            x += value;
        }
    }

    println!(
        "Part 1: {}",
        sum_signal_strengths(&signals, [20, 60, 100, 140, 180, 220])
    );

    println!("Part 2:");
    draw_signals(&signals);
}

fn sum_signal_strengths(signals: &Vec<i32>, cycles: [usize; 6]) -> i32 {
    let mut signal_strength_sum = 0;
    for i in cycles {
        signal_strength_sum += i as i32 * signals[i - 1];
    }

    return signal_strength_sum;
}

fn draw_signals(signals: &Vec<i32>) {
    for row in signals.chunks(40) {
        for (i, sprite_position) in row.into_iter().enumerate() {
            if i as i32 >= sprite_position - 1 && i as i32 <= sprite_position + 1 {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
