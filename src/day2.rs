fn solve1(contents: &str) -> i32 {
    let mut pos = (0, 0);
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let instruction = parts.next();
        let quantity = parts.next().unwrap().parse::<i32>().unwrap();
        match instruction {
            Some("forward") => pos.0 += quantity,
            Some("up") => pos.1 -= quantity,
            Some("down") => pos.1 += quantity,
            _ => panic!("bad input"),
        }
    }
    pos.0 * pos.1
}

fn solve2(contents: &str) -> i32 {
    let mut aim = 0;
    let mut pos = (0, 0);
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let instruction = parts.next();
        let quantity = parts.next().unwrap().parse::<i32>().unwrap();
        match instruction {
            Some("forward") => {
                pos.0 += quantity;
                pos.1 += quantity * aim;
            }
            Some("up") => aim -= quantity,
            Some("down") => aim += quantity,
            _ => panic!("bad input"),
        }
    }
    pos.0 * pos.1
}

pub fn run() {
    let timer = std::time::Instant::now();

    let contents = std::fs::read_to_string("inputs/day2").unwrap();

    let solution1 = solve1(&contents);
    let solution2 = solve2(&contents);

    println!(
        "solution 1 : {}, solution 2 : {}, {}us",
        solution1,
        solution2,
        timer.elapsed().as_micros()
    );
}
