use std::fs;
use std::iter::Iterator;

fn main() {
    let input = fs::read_to_string("src\\input.txt").unwrap();

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect();

    let (visible_trees, scenic_score) = count_scores(&grid);

    println!("Part 1: {}", visible_trees);
    println!("Part 2: {}", scenic_score);
}

fn count_scores(grid: &Vec<Vec<u32>>) -> (usize, usize) {
    let mut visible_trees = 0;
    let mut scenic_score = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let neighbour_trees = count_neighbour_trees(grid, i, j);

            // part 1
            if is_visible_tree(neighbour_trees) {
                visible_trees += 1;
            }

            // part 2
            let product = neighbour_trees.iter().map(|t| t.0).product::<usize>();
            if product > scenic_score {
                scenic_score = product;
            }
        }
    }

    return (visible_trees, scenic_score);
}

fn count_neighbour_trees(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> [(usize, bool); 4] {
    let mut trees: [(usize, bool); 4] = [(0, true); 4];

    trees[0] = count_trees(grid, i as i32, j as i32, 0, -1);
    trees[1] = count_trees(grid, i as i32, j as i32, 0, 1);
    trees[2] = count_trees(grid, i as i32, j as i32, -1, 0);
    trees[3] = count_trees(grid, i as i32, j as i32, 1, 0);

    return trees;
}

fn count_trees(grid: &Vec<Vec<u32>>, i: i32, j: i32, dx: i32, dy: i32) -> (usize, bool) {
    let mut tree_count: (usize, bool) = (0, true);
    let tree = grid[i as usize][j as usize];

    let mut x = i + dx;
    let mut y = j + dy;

    while x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
        tree_count.0 += 1;
        if grid[x as usize][y as usize] >= tree {
            tree_count.1 = false;
            break;
        }

        x += dx;
        y += dy;
    }

    return tree_count;
}

fn is_visible_tree(tree_distances: [(usize, bool); 4]) -> bool {
    return tree_distances.iter().any(|&(_, is_visible)| is_visible);
}
