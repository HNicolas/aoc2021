fn solve1(contents: &str) -> u64 {
    let gamma = contents
        .lines()
        .fold(vec![0; 12], |mut acc, curr| {
            for (i, c) in curr.char_indices() {
                if c == '1' {
                    acc[i] += 1;
                }
            }
            acc
        })
        .iter()
        .map(|&v| if v > 500 { 1 } else { 0 })
        .fold((0u64, 12), |(acc, index), curr| {
            (acc + curr * 2u64.pow(index - 1), index - 1)
        })
        .0;
    let epsilon = 2u64.pow(12) - gamma - 1;
    println!("{}, {}", gamma, epsilon);
    gamma * epsilon
}

pub fn run() {
    let timer = std::time::Instant::now();

    let contents = std::fs::read_to_string("inputs/day3").unwrap();

    let solution1 = solve1(&contents);
    let solution2 = "";

    println!(
        "solution 1 : {}, solution 2 : {}, {}us",
        solution1,
        solution2,
        timer.elapsed().as_micros()
    );
}
