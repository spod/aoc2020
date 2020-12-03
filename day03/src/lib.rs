use prob::Problem;
use std::io::BufRead;

pub struct Day03 {}

impl Problem for Day03 {
    fn new(&self, _input: &dyn BufRead) -> &'static dyn Problem {
        todo!()
    }

    fn part_1(&self) -> Result<i32, &str> {
        todo!()
    }

    fn part_2(&self) -> Result<i32, &str> {
        todo!()
    }
}

fn part1(input: Vec<&str>) -> i8 {
    let grid = parse_input(input);
    let x_move = 3;
    let y_move = 1;
    let wrap = grid[0].len();

    let mut count = 0;
    let mut x = 0;
    let mut y = 0;
    println!(
        "x, y at start: {},{}; grid[{}][{}]: {}",
        x, y, x, y, grid[y][x]
    );
    while y < grid.len() - 1 {
        x = (x + x_move) % wrap;
        y = y + y_move;
        if grid[y][x] == 1 {
            println!("moved to grid[{}][{}]: {} - Ow!", x, y, grid[y][x]);
        } else {
            println!("moved to grid[{}][{}]: {}", x, y, grid[y][x]);
        }
        count = count + grid[y][x];
    }
    println!("count: {}", count);
    count
}

// convert input to [[0,1,0...]....] where 1 is if input char is #
// in hindsight didn't need to waste time converting chars to 1 or 0 oh well
fn parse_input(input: Vec<&str>) -> Vec<Vec<i8>> {
    let mut grid = Vec::new();
    for line in input {
        let r = line
            .chars()
            .collect::<Vec<_>>()
            .iter()
            .map(|c| if c == &'#' { 1 } else { 0 })
            .collect::<Vec<i8>>();
        grid.push(r);
    }
    grid
}
fn part2(_: &str) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day03_test_parse_input() {
        let test_grid = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        let output = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        ];
        assert_eq!(parse_input(test_grid), output);
    }
    #[test]
    fn day03_part1_samples() {
        let test_grid = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        assert_eq!(part1(test_grid), 7);
    }

    #[test]
    fn day03_part2_samples() {
        let tests = vec![("sample1", ()), ("sample2", ()), ("sample3", ())];
        for t in tests {
            assert_eq!(part2(t.0), t.1);
        }
    }
}
