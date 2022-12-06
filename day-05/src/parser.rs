use std::str::Chars;

use crate::stack::Stack;

pub fn parse_crates(input: &str) -> Vec<Stack> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let stack_amount = (lines.pop().expect("No Stacks there :c").len() + 1) / 4;
    let mut stacks = vec![Stack::default(); stack_amount];

    lines
        .iter()
        .rev()
        .map(|line| line.chars())
        .for_each(|mut chars| {
            advance_by(&mut chars, 1);
            for stack in &mut stacks {
                let c = chars.next();
                match c {
                    Some(c) if c.is_alphabetic() => {
                        stack.put(c);
                    }
                    _ => {}
                }

                advance_by(&mut chars, 3);
            }
        });

    stacks
}

#[inline]
fn advance_by(not_self: &mut Chars, n: usize) {
    for _ in 0..n {
        not_self.next();
    }
}
