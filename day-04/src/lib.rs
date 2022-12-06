pub fn normal() {
    let input = include_str!("input.txt");
    let result = input
        .lines()
        .filter(|line| {
            let pairs = line
                .split(',')
                .map(|range_str| {
                    range_str
                        .split('-')
                        .map(|point| point.parse::<u32>().expect("Couldn't convert to number"))
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();

            #[inline]
            const fn inside(inner: (u32, u32), outer: (u32, u32)) -> bool {
                outer.0 <= inner.0 && inner.1 <= outer.1
            }

            let first = (pairs[0][0], pairs[0][1]);
            let second = (pairs[1][0], pairs[1][1]);

            inside(first, second) || inside(second, first)
        })
        .count();
    println!("{result}");
}
