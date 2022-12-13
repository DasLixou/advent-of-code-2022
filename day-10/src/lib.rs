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
    let mut cycle = 0;
    let mut result = 0;

    for instruction in input
        .lines()
        .map(|line| Instruction::from_str(line).expect("Coulnd't convert to Instruction"))
    {
        let mut check_cycle = |value| {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                println!("{cycle}");
                result += cycle * value;
            }
        };
        match instruction {
            Instruction::Noop => check_cycle(x),
            Instruction::AddX(value) => {
                check_cycle(x);
                check_cycle(x);
                x += value;
            }
        }
    }

    println!("Result: {result}");
}
