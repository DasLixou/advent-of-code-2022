use std::str::FromStr;

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::AddX(
                parts[1].parse::<i32>().expect("Couldn't parse to number"),
            )),
            _ => Err("Unknown Instruction".into()),
        }
    }
}

pub fn normal() {
    let input = include_str!("input.txt");
    let mut x: i32 = 1;
    let mut cycle_index = 0;
    let mut result = 0;

    for instruction in input
        .lines()
        .map(|line| Instruction::from_str(line).expect("Coulnd't convert to Instruction"))
    {
        let mut cycle = |value| {
            cycle_index += 1;
            if (cycle_index - 20) % 40 == 0 {
                result += cycle_index * value;
            }
        };

        match instruction {
            Instruction::Noop => cycle(x),
            Instruction::AddX(value) => {
                cycle(x);
                cycle(x);
                x += value;
            }
        }
    }

    println!("Result: {result}");
}

pub fn bonus() {
    let input = include_str!("input.txt");
    let mut x: i32 = 1;
    let mut cycle_index = 0;

    for instruction in input
        .lines()
        .map(|line| Instruction::from_str(line).expect("Coulnd't convert to Instruction"))
    {
        let mut cycle = |value: i32| {
            let line_index = cycle_index % 40;
            if (line_index - value).abs() <= 1 {
                print!("#");
            } else {
                print!(".")
            }
            cycle_index += 1;
            if (cycle_index % 40) == 0 {
                println!();
            }
        };

        match instruction {
            Instruction::Noop => {
                // cycle
                cycle(x);
            }
            Instruction::AddX(value) => {
                cycle(x);
                cycle(x);
                x += value;
            }
        }
    }
}
