use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let mut filesystem: HashMap<String, (u32, Vec<String>)> = HashMap::new();
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    parse_filesystem(&input, &mut filesystem);
    let used_space = calculate_directory_sizes(&filesystem, "/", &mut dir_sizes);

    println!(
        "Part 1: {}",
        sum_directories_under_limit(&dir_sizes, 100000)
    );

    let needed_space = (70000000 - used_space).abs_diff(30000000);
    println!(
        "Part 2: {}",
        dir_sizes
            .get(&find_directory_with_minimum_size(&dir_sizes, needed_space))
            .unwrap()
    );
}

fn parse_filesystem(input: &str, filesystem: &mut HashMap<String, (u32, Vec<String>)>) {
    let mut stack: Vec<String> = Vec::new();
    let root = String::from("/");

    for line in input.lines() {
        if line.contains("$ ls") {
            continue;
        }

        let top: &String = stack.last().unwrap_or(&root);
        if line.starts_with("$") {
            let dir = &line[5..];
            if dir == "/" {
                stack.push(root.clone());
                filesystem.insert(root.clone(), (0, Vec::new()));

                continue;
            } else if dir == ".." {
                stack.pop();
            } else {
                stack.push(format!("{}{}/", top, dir));
            }
        } else {
            let (x, y) = line.split_once(" ").unwrap();
            if x == "dir" {
                if !filesystem.contains_key(&format!("{}{}/", top, y)) {
                    filesystem.insert(format!("{}{}/", top, y), (0, Vec::new()));
                }

                let top_dir = filesystem.get_mut(top).unwrap();
                top_dir.1.push(format!("{}{}/", top, y));
            } else {
                let top_dir = filesystem.get_mut(top).unwrap();
                top_dir.1.push(x.to_string());
            }
        }
    }
}

fn calculate_directory_sizes(
    filesystem: &HashMap<String, (u32, Vec<String>)>,
    name: &str,
    dir_sizes: &mut HashMap<String, u32>,
) -> u32 {
    if !name.starts_with("/") {
        return name.parse::<u32>().unwrap();
    }

    let mut size = 0;
    for file in filesystem.get(name).unwrap().1.iter() {
        size += calculate_directory_sizes(filesystem, &file, dir_sizes);
    }

    dir_sizes.insert(name.to_string(), size);

    return size;
}

fn sum_directories_under_limit(dir_sizes: &HashMap<String, u32>, limit: u32) -> u32 {
    let mut sum = 0;
    for (_, size) in dir_sizes.iter() {
        if *size <= limit {
            sum += size;
        }
    }

    return sum;
}

fn find_directory_with_minimum_size(dir_sizes: &HashMap<String, u32>, size: u32) -> String {
    let mut min_size = u32::MAX;
    let mut min_dir = String::new();

    for (dir, dir_size) in dir_sizes.iter() {
        if *dir_size < min_size && *dir_size >= size {
            min_size = *dir_size;
            min_dir = dir.clone();
        }
    }

    return min_dir;
}
