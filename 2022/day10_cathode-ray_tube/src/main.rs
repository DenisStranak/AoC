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
    draw_pixels(get_pixels(&signals));
}

fn sum_signal_strengths(signals: &Vec<i32>, cycles: [usize; 6]) -> i32 {
    let mut signal_strength_sum = 0;
    for i in cycles {
        signal_strength_sum += i as i32 * signals[i - 1];
    }

    return signal_strength_sum;
}

fn get_pixels(signals: &Vec<i32>) -> Vec<char> {
    let mut pixels: Vec<char> = Vec::new();
    for chunk in signals.chunks(40) {
        for (i, sprite_position) in chunk.into_iter().enumerate() {
            if i as i32 >= sprite_position - 1 && i as i32 <= sprite_position + 1 {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
        }
    }

    return pixels;
}

fn draw_pixels(pixels: Vec<char>) {
    for row in pixels.chunks(40) {
        println!("{}", row.into_iter().collect::<String>());
    }
}
