use prob::Problem;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

pub struct Day05 {}

impl Problem for Day05 {
    fn part_1(&self) -> Result<i32, &str> {
        todo!()
    }

    fn part_2(&self) -> Result<i32, &str> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct Seat {
    row: i32,
    column: i32,
    id: i32,
}

fn decode_seat(pass: &str) -> Seat {
    let mut row_min = 0;
    let mut row_max = 127;
    let mut col_min = 0;
    let mut col_max = 7;
    for op in pass.chars() {
        // println!("Before - op: {}, row_min: {}, row_max: {}, col_min: {}, col_max: {}", op, row_min, row_max, col_min, col_max);
        match op {
            'F' => row_max = row_max - ((row_max - row_min) / 2) - 1,
            'B' => row_min = row_min + ((row_max - row_min) / 2) + 1,
            'L' => col_max = col_max - ((col_max - col_min) / 2) - 1,
            'R' => col_min = col_min + ((col_max - col_min) / 2) + 1,
            _ => (),
        }
        // println!("After - op: {}, row_min: {}, row_max: {}, col_min: {}, col_max: {}", op, row_min, row_max, col_min, col_max);
    }
    Seat {
        row: row_min,
        column: col_min,
        id: (row_min * 8) + col_min,
    }
}
fn part1(input: Vec<String>) -> i32 {
    let mut max = 0;
    for line in &input {
        let s = decode_seat(&line);
        if s.id > max {
            max = s.id
        }
    }
    max
}

fn part2(input: Vec<String>) -> i32 {
    let mut ids: Vec<i32> = vec![];
    for line in &input {
        let s = decode_seat(&line);
        if s.row != 1 && s.row != 127 {
            ids.push(s.id)
        }
    }
    ids.sort();
    let min = ids[0];
    let max = ids[ids.len() - 1];

    let used_seats: HashSet<i32> = ids.into_iter().collect();
    let all_seats: HashSet<i32> = HashSet::from_iter(min..=max);
    let diff = all_seats.difference(&used_seats);
    println!("diff: {:?}", diff);
    *diff.collect::<Vec<&i32>>()[0]
}

fn load_input() -> Vec<String> {
    let inlines = fs::read_to_string("../inputs/day05").unwrap();
    let result = inlines.lines().map(|s| String::from(s)).collect();
    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn decode_seat_samples() {
        let tests = vec![
            (
                "FBFBBFFRLR",
                Seat {
                    row: 44,
                    column: 5,
                    id: 357,
                },
            ),
            (
                "BFFFBBFRRR",
                Seat {
                    row: 70,
                    column: 7,
                    id: 567,
                },
            ),
            (
                "FFFBBBFRRR",
                Seat {
                    row: 14,
                    column: 7,
                    id: 119,
                },
            ),
            (
                "BBFFBBFRLL",
                Seat {
                    row: 102,
                    column: 4,
                    id: 820,
                },
            ),
        ];
        for t in tests {
            assert_eq!(decode_seat(t.0), t.1)
        }
    }

    #[test]
    fn day04_part1_sample() {
        let input = vec![
            String::from("FBFBBFFRLR"),
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ];
        assert_eq!(part1(input), 820);
    }

    #[test]
    fn day04_part1_real() {
        let input = load_input();
        assert_eq!(part1(input), 989);
    }

    #[test]
    fn day04_part2_real() {
        let input = load_input();
        assert_eq!(part2(input), 548);
    }
}
