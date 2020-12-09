use prob::Problem;
use std::fs;

pub struct Day02 {}

impl Problem for Day02 {
    fn part_1(&self) -> Result<i32, &str> {
        todo!()
    }

    fn part_2(&self) -> Result<i32, &str> {
        todo!()
    }
}

fn valid_password(input: &str) -> bool {
    let (min, max, chr, passwd) = parse_pwd_record(input);
    let m: Vec<&str> = passwd.matches(chr).collect();
    m.len() >= min && m.len() <= max
}

fn valid_password_real(input: &str) -> bool {
    let (p1, p2, chr, passwd) = parse_pwd_record(input);
    let m1 = passwd.chars().nth(p1 - 1).unwrap();
    let m2 = passwd.chars().nth(p2 - 1).unwrap();
    //println!("for input: {}, chr: {}; m1: {} & m2: {}", &passwd, &chr, &m1, &m2);
    (m1 == chr) ^ (m2 == chr)
}

// Given policy "1-3 a: abcde" returns (1, 3, "a", "abcde")
fn parse_pwd_record(pwd: &str) -> (usize, usize, char, &str) {
    // Ths should probably be a regex, sorry, "Good enough is"
    let p: Vec<&str> = pwd.split(": ").collect(); // p = ("1-3 a", "abcde")
    let passwd = p[1];
    let p: Vec<&str> = p[0].split(" ").collect(); // p = ("1-3", "a")
    let chr = p[1].chars().nth(0).unwrap();
    let p: Vec<&str> = p[0].split("-").collect(); // p = ("1", "3")
    let min = p[0].parse::<usize>().unwrap();
    let max = p[1].parse::<usize>().unwrap();
    (min, max, chr, passwd)
}

fn part1() -> usize {
    let inlines = fs::read_to_string("../inputs/day02_part1").unwrap();
    let input: Vec<&str> = inlines.lines().collect();
    input.iter().filter(|v| valid_password(v)).count()
}

fn part2() -> usize {
    let inlines = fs::read_to_string("../inputs/day02_part1").unwrap();
    let input: Vec<&str> = inlines.lines().collect();
    input.iter().filter(|v| valid_password_real(v)).count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day02_parse_pwd_record() {
        let tests = vec![
            ("1-3 a: abcde", (1, 3, 'a', "abcde")),
            ("1-3 b: cdefg", (1, 3, 'b', "cdefg")),
            ("2-9 c: ccccccccc", (2, 9, 'c', "ccccccccc")),
        ];
        for t in tests {
            assert_eq!(parse_pwd_record(t.0), t.1);
        }
    }

    #[test]
    fn day02_valid_password() {
        let tests = vec![
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", true),
        ];
        for t in tests {
            assert_eq!(valid_password(t.0), t.1);
        }
    }

    #[test]
    fn day02_valid_password_real() {
        let tests = vec![
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", false),
        ];
        for t in tests {
            assert_eq!(valid_password_real(t.0), t.1);
        }
    }
    #[test]
    fn day02_part1_real() {
        let result = part1();
        println!("part1() returned: {}\n", result);
        assert_eq!(result, 643);
    }

    #[test]
    fn day02_part2_real() {
        let result = part2();
        println!("part2() returned: {}\n", result);
        assert_eq!(result, 388);
    }
}
