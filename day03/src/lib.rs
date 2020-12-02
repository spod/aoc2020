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

fn part1(_: &str) -> () {
    ()
}

fn part2(_: &str) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day03_part1_samples() {
        let tests = vec![("sample1", ()), ("sample2", ()), ("sample3", ())];
        for t in tests {
            assert_eq!(part1(t.0), t.1);
        }
    }

    #[test]
    fn day03_part2_samples() {
        let tests = vec![("sample1", ()), ("sample2", ()), ("sample3", ())];
        for t in tests {
            assert_eq!(part2(t.0), t.1);
        }
    }
}
