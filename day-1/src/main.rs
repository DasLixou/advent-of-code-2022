mod parser;

fn main() {
    let input = include_str!("input.txt");
    let reindeers = parser::parse(input);
    let max_calories = reindeers.iter().max().expect("No reindeer found :c");
    println!("Result: {}", max_calories);
}
