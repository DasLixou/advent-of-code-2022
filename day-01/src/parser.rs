use std::io::{BufRead, Cursor};

pub fn parse(input: impl Into<String>) -> Vec<u32> {
    let mut cursor = Cursor::new(input.into());
    let mut buf = String::new();
    let mut calories: u32 = 0;
    let mut reindeers = Vec::<u32>::new();
    loop {
        if let Ok(0) = cursor.read_line(&mut buf) {
            break;
        }

        buf.retain(|c| !c.is_whitespace());

        if buf.is_empty() {
            reindeers.push(calories);
            calories = 0;
        } else {
            calories += buf
                .parse::<u32>()
                .expect(format!("Line '{}' isn't a number", buf).as_str());
            buf.clear();
        }
    }
    reindeers
}
