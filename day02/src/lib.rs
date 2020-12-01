use prob::Problem;
use std::io::BufRead;

pub struct Day02 {
    input: Vec<i32>,
}

impl Problem for Day02 {
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

fn part1(input: &Vec<i32>) -> i32 {
    42
}

fn part2(input: &Vec<i32>) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn day02_part1_sample() {
        let input = vec![1, 2, 3];
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn day02_part1_real() {
        let input = vec![1, 2, 3];
        let result = part1(&input);
        println!("part1({:?}) returned: {}\n", input, result);
        assert_eq!(result, 42);
    }

    #[test]
    fn day02_part2_sample() {
        let input = vec![1, 2, 3];
        assert_eq!(part2(&input), 42);
    }

    #[test]
    fn day02_part2_real() {
        let input = vec![1, 2, 3];
        let result = part1(&input);
        println!("part2({:?}) returned: {}\n", input, result);
        assert_eq!(result, 42);
    }
}
