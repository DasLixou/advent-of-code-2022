type Tree = (i32, bool);

pub fn normal() {
    let input = include_str!("input.txt");
    let mut trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .map(|height| (height, false))
                .collect::<Vec<Tree>>()
        })
        .collect::<Vec<Vec<Tree>>>();

    // left to right
    for y in 0..trees.len() {
        let mut max_height = -1;
        for x in 0..trees[y].len() {
            if trees[y][x].0 > max_height {
                max_height = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
    }

    // right to left
    for y in 0..trees.len() {
        let mut max_height = -1;
        for x in (0..trees[y].len()).rev() {
            if trees[y][x].0 > max_height {
                max_height = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
    }

    // top to bottom
    for x in 0..trees[0].len() {
        let mut max_height = -1;
        for y in 0..trees.len() {
            if trees[y][x].0 > max_height {
                max_height = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
    }

    // bottom to top
    for x in 0..trees[0].len() {
        let mut max_height = -1;
        for y in (0..trees.len()).rev() {
            if trees[y][x].0 > max_height {
                max_height = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
    }

    let result = trees
        .iter()
        .map(|row| row.iter().filter(|tree| tree.1).count())
        .sum::<usize>();

    println!("Result: {result}");
}

pub fn bonus() {
    let input = include_str!("input.txt");
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut max_score = 0;
    for tree_y in 0..trees.len() {
        for tree_x in 0..trees[tree_y].len() {
            let tree_height = trees[tree_y][tree_x];
            let max_x = trees[tree_y].len();
            let max_y = trees.len();

            // right
            let mut right = 0;
            for x in (tree_x + 1)..max_x {
                if trees[tree_y][x] < tree_height {
                    right += 1;
                } else {
                    right += 1;
                    break;
                }
            }

            // left
            let mut left = 0;
            for x in (0..tree_x).rev() {
                if trees[tree_y][x] < tree_height {
                    left += 1;
                } else {
                    left += 1;
                    break;
                }
            }

            // top
            let mut top = 0;
            for y in (0..tree_y).rev() {
                if trees[y][tree_x] < tree_height {
                    top += 1;
                } else {
                    top += 1;
                    break;
                }
            }

            // bottom
            let mut bottom = 0;
            for y in (tree_y + 1)..max_y {
                if trees[y][tree_x] < tree_height {
                    bottom += 1;
                } else {
                    bottom += 1;
                    break;
                }
            }

            let tree_score = right * left * top * bottom;
            if tree_score > max_score {
                max_score = tree_score;
            }
        }
    }

    println!("Result: {max_score}");
}
