fn solve(depths: &Vec<u16>) -> u16 {
    let mut current_depth = depths[0];
    let mut steps = 0;

    for &depth in &depths[1..] {
        if depth > current_depth {
            steps += 1;
        }
        current_depth = depth;
    }
    steps
}

fn to_depths(lines: &Vec<&str>, windows_size: usize) -> Vec<u16> {
    lines.windows(windows_size).map(|slice| slice.iter().fold(0, |acc, &curr| acc + curr.parse::<u16>().unwrap())).collect()
}

pub fn run() {
    let timer = std::time::Instant::now();

    let contents = std::fs::read_to_string("inputs/day1").unwrap();
    let lines = contents.lines().collect::<Vec<_>>();
    let solution1 = solve(&to_depths(&lines, 1));
    let solution2 = solve(&to_depths(&lines, 3));

    println!("solution 1 : {}, solution 2 : {}, {}us", solution1, solution2, timer.elapsed().as_micros());
}
