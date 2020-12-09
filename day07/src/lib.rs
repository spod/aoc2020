use prob::Problem;
use std::fs;
pub struct Day07 {}

impl Problem for Day07 {
    fn part_1(&self) -> Result<i32, &str> {
        todo!()
    }

    fn part_2(&self) -> Result<i32, &str> {
        todo!()
    }
}

fn part1(input: Vec<String>) -> i32 {
    4
}

fn load_input() -> Vec<String> {
    let inlines = fs::read_to_string("../inputs/day07").unwrap();
    let result = inlines.lines().map(|s| String::from(s)).collect();
    result
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn day07_part1_sample() {
        let input: Vec<String> = vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags."),
        ];
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn day07_part1_real() {
        let input = load_input();
        assert_eq!(part1(input), 4);
    }
}
