/// (x(i) + ... + x(i-n)) - (x(i-1) + ... + x(i-n-1)) = x(i) - x(i-n-1)
/// where n is the size of the window
/// i - n - 1 >= 0, i >= n + 1
fn solve(depths: &Vec<u16>, window_size: usize) -> u16 {
    let mut steps = 0;

    for i in (1 + window_size)..depths.len() {
        if depths[i] > depths[i - window_size - 1] {
            steps += 1;
        }
    }
    steps
}

pub fn run() {
    let timer = std::time::Instant::now();

    let contents = std::fs::read_to_string("inputs/day1").unwrap();
    let depths = contents.lines().map(|s| s.parse().unwrap()).collect();
    let solution1 = solve(&depths, 0);
    let solution2 = solve(&depths, 2);

    println!(
        "solution 1 : {}, solution 2 : {}, {}us",
        solution1,
        solution2,
        timer.elapsed().as_micros()
    );
}
