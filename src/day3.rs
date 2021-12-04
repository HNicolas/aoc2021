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

fn split(values: Vec<Vec<char>>, index: usize) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut ones = vec![];
    let mut zeros = vec![];
    for chars in values {
        if chars[index] == '1' {
            ones.push(chars);
        } else {
            zeros.push(chars);
        }
    }
    (ones, zeros)
}

fn to_decimal(value: &Vec<char>) -> u64 {
    let mut result = 0;
    for i in 0..value.len() {
        if value[i] == '1' {
            result += 2u64.pow(11 - (i as u32));
        }
    }
    result
}

fn solve2(contents: &str) -> u64 {
    let values = contents
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (ones, zeros) = split(values, 0);
    let (mut oxigen, mut co2) = if ones.len() >= zeros.len() {
        (ones, zeros)
    } else {
        (zeros, ones)
    };

    let mut index = 1;
    while oxigen.len() > 1 && index < 12 {
        let (ones, zeros) = split(oxigen, index);
        oxigen = if ones.len() >= zeros.len() {
            ones
        } else {
            zeros
        };
        index += 1;
    }

    index = 1;
    while co2.len() > 1 && index < 12 {
        let (ones, zeros) = split(co2, index);
        co2 = if zeros.len() <= ones.len() {
            zeros
        } else {
            ones
        };
        index += 1;
    }

    to_decimal(&oxigen[0]) * to_decimal(&co2[0])
}

pub fn run() {
    let timer = std::time::Instant::now();

    let contents = std::fs::read_to_string("inputs/day3").unwrap();

    let solution1 = solve1(&contents);
    let solution2 = solve2(&contents);

    println!(
        "solution 1 : {}, solution 2 : {}, {}us",
        solution1,
        solution2,
        timer.elapsed().as_micros()
    );
}
