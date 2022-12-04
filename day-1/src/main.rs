mod parser;

fn main() {
    let input = include_str!("input.txt");
    let mut reindeers = parser::parse(input);
    reindeers.sort_by(|a, b| b.cmp(a));
    println!("Result: {}", reindeers[0] + reindeers[1] + reindeers[2]);
}
