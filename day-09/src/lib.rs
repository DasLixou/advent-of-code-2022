use std::collections::HashSet;

use glam::IVec2;

#[derive(Debug)]
enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Move {
    fn new(instruction: &str, steps: &str) -> Self {
        let parsed_steps = steps.parse::<u32>().expect("Coulnd't convert steps to u32");
        match instruction
            .chars()
            .next()
            .expect("Instruction string is empty")
        {
            'U' => Move::Up(parsed_steps),
            'D' => Move::Down(parsed_steps),
            'L' => Move::Left(parsed_steps),
            'R' => Move::Right(parsed_steps),
            _ => panic!("Unknown instruction"),
        }
    }

    #[inline]
    fn data(&self) -> (u32, IVec2) {
        match *self {
            Move::Up(steps) => (steps, IVec2::Y),
            Move::Down(steps) => (steps, IVec2::NEG_Y),
            Move::Left(steps) => (steps, IVec2::NEG_X),
            Move::Right(steps) => (steps, IVec2::X),
        }
    }
}

pub fn normal() {
    let input = include_str!("input.txt");
    let moves = input
        .lines()
        .map(|line| {
            let data = line.split(' ').collect::<Vec<&str>>();
            Move::new(data[0], data[1])
        })
        .collect::<Vec<Move>>();

    let mut head = IVec2::new(0, 0);
    let mut tail = IVec2::new(0, 0);
    let mut visited_fields = HashSet::new();

    visited_fields.insert(tail); // insert start field

    moves.iter().for_each(|m| {
        let (steps, direction) = m.data();
        (0..steps).for_each(|_| {
            head += direction;
            // tail movement
            if (head - tail).abs().cmpgt(IVec2::ONE).any() {
                // horizontal movement
                if tail.y == head.y {
                    let distance = head.x - tail.x;
                    if distance > 0 {
                        tail.x += distance - 1;
                    } else {
                        tail.x += distance + 1;
                    }
                }
                // vertical movement
                else if tail.x == head.x {
                    let distance = head.y - tail.y;
                    if distance > 0 {
                        tail.y += distance - 1;
                    } else {
                        tail.y += distance + 1;
                    }
                }
                // diagonal movement
                else {
                    let tail_left = tail.x < head.x;
                    let tail_below = tail.y < head.y;
                    tail += match (tail_left, tail_below) {
                        (true, true) => IVec2::new(1, 1),
                        (false, true) => IVec2::new(-1, 1),
                        (true, false) => IVec2::new(1, -1),
                        (false, false) => IVec2::new(-1, -1),
                    };
                }
            }
            visited_fields.insert(tail);
        })
    });

    let result = visited_fields.len();
    println!("Result: {result}");
}

pub fn bonus() {
    let input = include_str!("input.txt");
    let moves = input
        .lines()
        .map(|line| {
            let data = line.split(' ').collect::<Vec<&str>>();
            Move::new(data[0], data[1])
        })
        .collect::<Vec<Move>>();

    let mut head = IVec2::new(0, 0);
    let mut ropes = [IVec2::new(0, 0); 9];
    let mut visited_fields = HashSet::new();

    visited_fields.insert(ropes[8]); // insert start field

    moves.iter().for_each(|m| {
        let (steps, direction) = m.data();
        (0..steps).for_each(|_| {
            head += direction;
            // rope movement
            for i in 0..ropes.len() {
                let target = match i {
                    h if h == 0 => head,
                    _ => ropes[i - 1],
                };
                let mut rope = &mut ropes[i];
                if (target - *rope).abs().cmpgt(IVec2::ONE).any() {
                    // horizontal movement
                    if rope.y == target.y {
                        let distance = target.x - rope.x;
                        if distance > 0 {
                            rope.x += distance - 1;
                        } else {
                            rope.x += distance + 1;
                        }
                    }
                    // vertical movement
                    else if rope.x == target.x {
                        let distance = target.y - rope.y;
                        if distance > 0 {
                            rope.y += distance - 1;
                        } else {
                            rope.y += distance + 1;
                        }
                    }
                    // diagonal movement
                    else {
                        let tail_left = rope.x < target.x;
                        let tail_below = rope.y < target.y;
                        *rope += match (tail_left, tail_below) {
                            (true, true) => IVec2::new(1, 1),
                            (false, true) => IVec2::new(-1, 1),
                            (true, false) => IVec2::new(1, -1),
                            (false, false) => IVec2::new(-1, -1),
                        };
                    }
                }
                if i == 8 {
                    visited_fields.insert(*rope);
                }
            }
        })
    });

    let result = visited_fields.len();
    println!("Result: {result}");
}
