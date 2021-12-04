use std::collections::{HashMap, HashSet};

type Line = usize;
type Column = usize;

struct Grid {
    indices: HashMap<u8, (Line, Column)>,
    lines: Vec<HashSet<u8>>,
    columns: Vec<HashSet<u8>>,
}

impl Grid {
    fn new(vec: Vec<Vec<u8>>) -> Self {
        let mut indices = HashMap::new();
        let mut lines = vec![HashSet::new(); 5];
        let mut columns = vec![HashSet::new(); 5];
        for line in 0..vec.len() {
            for column in 0..vec[line].len() {
                let value = vec[line][column];
                indices.insert(value, (line, column));
                lines[line].insert(value);
                columns[column].insert(value);
            }
        }

        Self {
            indices,
            lines,
            columns,
        }
    }
}

struct Game {
    numbers: Vec<u8>,
    grids: Vec<Grid>,
}

impl Game {
    fn new(contents: &str) -> Self {
        let mut lines = contents.lines();
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let mut grids = vec![];

        let mut grid = vec![];
        for line in lines {
            if line == "" {
                continue;
            }

            let line = line
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            grid.push(line);

            if grid.len() == 5 {
                grids.push(Grid::new(grid));
                grid = vec![];
            }
        }

        Self { numbers, grids }
    }

    fn solve1(&mut self) -> u64 {
        for number in self.numbers.iter() {
            for grid in self.grids.iter_mut() {
                if let Some(&(line, column)) = grid.indices.get(number) {
                    grid.indices.remove(number);
                    grid.lines[line].remove(number);
                    grid.columns[column].remove(number);
                    if grid.lines[line].len() == 0 || grid.columns[column].len() == 0 {
                        // solve
                        return (*number as u64)
                            * grid
                                .indices
                                .keys()
                                .fold(0, |acc, &curr| acc + (curr as u64));
                    }
                }
            }
        }
        panic!("no solution found");
    }

    fn solve2(&mut self) -> u64 {
        let mut solved_grids = HashSet::new();
        let mut score = 0;
        for number in self.numbers.iter() {
            for grid_index in 0..self.grids.len() {
                if solved_grids.contains(&grid_index) {
                    continue;
                }
                let grid = &mut self.grids[grid_index];
                if let Some(&(line, column)) = grid.indices.get(number) {
                    grid.indices.remove(number);
                    grid.lines[line].remove(number);
                    grid.columns[column].remove(number);
                    if grid.lines[line].len() == 0 || grid.columns[column].len() == 0 {
                        score = (*number as u64)
                        * grid
                            .indices
                            .keys()
                            .fold(0, |acc, &curr| acc + (curr as u64));
                        solved_grids.insert(grid_index);
                    }
                }
            }
        }
        score
    }
}

pub fn run() {
    let timer = std::time::Instant::now();

    let contents = std::fs::read_to_string("inputs/day4").unwrap();

    let solution1 = Game::new(&contents).solve1();
    let solution2 = Game::new(&contents).solve2();

    println!(
        "solution 1 : {}, solution 2 : {}, {}us",
        solution1,
        solution2,
        timer.elapsed().as_micros()
    );
}
