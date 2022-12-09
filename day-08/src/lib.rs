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
