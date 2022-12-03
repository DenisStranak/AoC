use std::fs;

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let mut rucksack_priority = 0;
    let mut badge_priority = 0;

    let mut count = 0;
    let mut group: Vec<&str> = vec![];
    for line in input.lines() {
        count += 1;
        group.push(line);

        rucksack_priority += get_rucksack_priority(line);

        if count == 3 {
            badge_priority += get_group_badge_priority(&group);

            count = 0;
            group.clear();
        }
    }

    println!("First part: {}", rucksack_priority);
    println!("Second part: {}", badge_priority);
}

fn get_rucksack_priority(rucksack: &str) -> i32 {
    let mut priority = 0;

    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let right_items = right.chars().collect::<std::collections::HashSet<_>>();
    for item in right_items {
        if left.contains(item) {
            priority += get_item_priority(item);
        }
    }

    return priority;
}

fn get_group_badge_priority(group: &Vec<&str>) -> i32 {
    let mut priority = 0;

    let rucksack = group[0].chars().collect::<std::collections::HashSet<_>>();
    for item in rucksack {
        if group[1].contains(item) && group[2].contains(item) {
            priority += get_item_priority(item);
        }
    }

    return priority;
}

fn get_item_priority(item: char) -> i32 {
    return if item.is_lowercase() {
        item as i32 - 96
    } else {
        item as i32 - 38
    };
}
