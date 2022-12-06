use iter_tools::Itertools;

mod parser;
pub mod stack;

pub fn normal() {
    let text = include_str!("input.txt");
    let text_parts = text.split("\n\n").collect::<Vec<&str>>();
    let (input, instructions) = (text_parts[0], text_parts[1]);

    let mut crates = parser::parse_crates(input);

    instructions.lines().for_each(|line| {
        let info = line
            .split(char::is_whitespace)
            .filter(|s| s.chars().all(|c| c.is_numeric()))
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();

        let source = info[1] - 1;
        let target = info[2] - 1;

        for _ in 0..info[0] {
            let cratee = crates[source].take();
            crates[target].put(cratee)
        }
    });

    let mut result = String::with_capacity(crates.len());
    crates
        .iter_mut()
        .for_each(|stack| result.push(stack.take()));

    println!("Result: {result}");
}
