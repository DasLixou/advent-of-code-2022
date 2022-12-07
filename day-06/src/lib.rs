pub fn normal() {
    let input = include_str!("input.txt");
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find_map(|(i, seq)| {
            if seq
                .iter()
                .enumerate()
                .any(|(j, c)| seq[j + 1..].contains(c))
            {
                None
            } else {
                Some(i + 4)
            }
        })
        .expect("No marker found :c");

    println!("Result: {result}");
}
