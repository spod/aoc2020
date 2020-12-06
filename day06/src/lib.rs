use prob::Problem;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
pub struct Day06 {}

impl Problem for Day06 {
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

fn load_input() -> Vec<String> {
    let inlines = fs::read_to_string("../inputs/day06").unwrap();
    let result = inlines.lines().map(|s| String::from(s)).collect();
    result
}

fn batch_group_counts(input: Vec<String>) -> Vec<HashMap<char, i32>> {
    todo!();
}

fn part1(input: Vec<String>) -> i32 {
    let mut buf: Vec<String> = vec![];
    let mut count: i32 = 0;
    for line in input {
        if line == "" {
            // new group
            let mut group_counts = HashMap::new();
            for line in &buf {
                for c in line.chars() {
                    let cnt = group_counts.entry(c).or_insert(0);
                    *cnt += 1;
                }
            }
            count += group_counts.keys().len() as i32;
            buf = vec![];
        } else {
            buf.push(line);
        }
    }
    // deal with last record
    if buf.len() > 0 {
        let mut group_counts = HashMap::new();
        for line in &buf {
            for c in line.chars() {
                let cnt = group_counts.entry(c).or_insert(0);
                *cnt += 1;
            }
        }
        count += group_counts.keys().len() as i32;
    }
    count
}

fn part2(input: Vec<String>) -> i32 {
    6
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn day06_part1_sample() {
        let input: Vec<String> = vec![
            String::from("abc"),
            String::from(""),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from(""),
            String::from("ab"),
            String::from("ac"),
            String::from(""),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from(""),
            String::from("b"),
        ];
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn day06_part1_real() {
        let input = load_input();
        assert_eq!(part1(input), 6310);
    }

    #[test]
    fn day06_part2_sample() {
        let input: Vec<String> = vec![
            String::from("abc"),
            String::from(""),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from(""),
            String::from("ab"),
            String::from("ac"),
            String::from(""),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from(""),
            String::from("b"),
        ];
        assert_eq!(part2(input), 6);
    }
}
