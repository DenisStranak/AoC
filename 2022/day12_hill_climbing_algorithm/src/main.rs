use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("src\\input.txt").expect("Unable to read file");

    let (graph, start, end) = create_graph(&input);

    let mut shortest_path = std::usize::MAX;
    for x in 0..graph.len() {
        for y in 0..graph[0].len() {
            if graph[x][y] != 0 {
                continue;
            }

            let current_path = find_shortest_path(&graph, &(x as i32, y as i32), &end);
            if current_path < shortest_path {
                shortest_path = current_path;
            }
        }
    }

    println!("Part 1: {}", find_shortest_path(&graph, &start, &end));
    println!("Part 2: {}", shortest_path);
}

fn find_shortest_path(graph: &Vec<Vec<i32>>, start: &(i32, i32), end: &(i32, i32)) -> usize {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back(*start);

    let mut visited = vec![vec![false; graph[0].len()]; graph.len()];
    visited[start.0 as usize][start.1 as usize] = true;

    let mut previous = vec![vec![(-1, -1); graph[0].len()]; graph.len()];

    while let Some((cx, cy)) = queue.pop_front() {
        let neighbors = vec![(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)];
        for (x, y) in neighbors {
            if x < 0
                || x >= graph.len() as i32
                || y < 0
                || y >= graph[0].len() as i32
                || visited[x as usize][y as usize]
                || graph[x as usize][y as usize] > graph[cx as usize][cy as usize] + 1
            {
                continue;
            }

            if (x, y) == *end {
                queue.clear();
                previous[x as usize][y as usize] = (cx, cy);
                break;
            }

            queue.push_back((x, y));
            visited[x as usize][y as usize] = true;
            previous[x as usize][y as usize] = (cx, cy);
        }
    }

    let mut path_length = 0;
    let mut current = *end;
    while current != *start {
        if current == (-1, -1) {
            return std::usize::MAX;
        }

        current = previous[current.0 as usize][current.1 as usize];
        path_length += 1;
    }

    return path_length;
}

fn create_graph(input: &String) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    let mut graph = Vec::new();
    let (mut start, mut end) = ((0, 0), (0, 0));

    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (i as i32, j as i32);
                    row.push(0);
                }
                'E' => {
                    end = (i as i32, j as i32);
                    row.push(25);
                }
                _ => row.push(c as i32 - 97),
            }
        }
        graph.push(row);
    }

    return (graph, start, end);
}
