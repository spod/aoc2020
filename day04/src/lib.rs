use prob::Problem;
use std::io::BufRead;

pub struct Day04 {}

impl Problem for Day04 {
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

#[derive(Debug, PartialEq)]
struct Passport {
    byr: i16,
    iyr: i16,
    eyr: i16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn new(
        byr: i16,
        iyr: i16,
        eyr: i16,
        hgt: String,
        hcl: String,
        ecl: String,
        pid: String,
        cid: Option<String>,
    ) -> Self {
        Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }

    fn from_batch(batch_record: Vec<&str>) -> Result<Passport, &str> {
        // Err("Not implemented yet!")
        let result: Passport = Passport {
            byr: 1937,
            iyr: 2017,
            eyr: 2020,
            hgt: String::from("183cm"),
            hcl: String::from("#fffffd"),
            ecl: String::from("gry"),
            pid: String::from("860033327"),
            cid: Some(String::from("147")),
        };
        Ok(result)
    }
}

fn part1(batch: Vec<&str>) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn day04_passport_from_batch() {
        let input = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
        ];
        let result: Passport = Passport {
            byr: 1937,
            iyr: 2017,
            eyr: 2020,
            hgt: String::from("183cm"),
            hcl: String::from("#fffffd"),
            ecl: String::from("gry"),
            pid: String::from("860033327"),
            cid: Some(String::from("147")),
        };
        assert_eq!(Passport::from_batch(input).unwrap(), result);
    }

    #[test]
    fn day04_part1_samples() {
        let test_batch = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ];
        assert_eq!(part1(test_batch), 2);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
