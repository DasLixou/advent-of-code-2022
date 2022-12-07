pub fn normal() {
    let input = include_str!("input.txt");
    const LOOK_AHEAD: usize = 4;
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(LOOK_AHEAD)
        .enumerate()
        .find_map(|(i, seq)| {
            if seq
                .iter()
                .enumerate()
                .any(|(j, c)| seq[j + 1..].contains(c))
            {
                None
            } else {
                Some(i + LOOK_AHEAD)
            }
        })
        .expect("No marker found :c");

    println!("Result: {result}");
}

pub fn bonus() {
    let input = include_str!("input.txt");
    const LOOK_AHEAD: usize = 14;
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(LOOK_AHEAD)
        .enumerate()
        .find_map(|(i, seq)| {
            if seq
                .iter()
                .enumerate()
                .any(|(j, c)| seq[j + 1..].contains(c))
            {
                None
            } else {
                Some(i + LOOK_AHEAD)
            }
        })
        .expect("No marker found :c");

    println!("Result: {result}");
}
