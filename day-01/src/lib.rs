mod parser;

pub fn normal() {
    let input = include_str!("input.txt");
    let reindeers = parser::parse(input);
    let result: &u32 = reindeers.iter().max().expect("No reindeer found :c");
    println!("Result: {result}");
}

pub fn bonus() {
    let input = include_str!("input.txt");
    let mut reindeers = parser::parse(input);
    reindeers.sort_by(|a, b| b.cmp(a));
    let result: u32 = reindeers.iter().take(3).sum();
    println!("Result: {result}");
}
