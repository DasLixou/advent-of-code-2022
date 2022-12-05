use std::collections::HashMap;

pub fn normal() {
    let input = include_str!("input.txt");

    let scores: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    let result: usize = input
        .lines()
        .map(|line| {
            let compartment_size = line.len() / 2;
            let (l_comp, r_comp) = (&line[..compartment_size], &line[compartment_size..]);
            let item = l_comp
                .chars()
                .find(|i| r_comp.contains(*i))
                .expect("No item type that could be found in both compartments");
            scores.get(&item).unwrap()
        })
        .sum();
    println!("Result: {result}");
}
