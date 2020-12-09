use day01::Day01;
use day08::Day08;
use prob::Problem;

pub struct Config {
    pub day: i32,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // skip args[0] - binary name
        args.next();
        let day = match args.next() {
            Some(day) => day,
            None => return Err("Day is a required argument (integer)."),
        };
        let day = day.parse::<i32>().unwrap();
        if day <= 0 || day >= 31 {
            return Err("Day must be an integer between 1 and 31.");
        }
        Ok(Config { day })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    match config.day {
        8 => {
            println!("Day 08");
            let prob: Day08 = day08::Day08 {};
            let p1 = prob.part_1();
            match p1 {
                Ok(solution) => println!("part 1: {}", solution),
                Err(e) => println!("part 1 error: {}", e),
            }
            let p2 = prob.part_2();
            match p2 {
                Ok(solution) => println!("part 2: {}", solution),
                Err(e) => println!("part 2 error: {}", e),
            }
            Ok(())
        }
        1..=31 => {
            println!("TODO run day {} parts 1 & 2", config.day);
            Ok(())
        }
        _ => Err("Solution not implemented yet!"),
    }
}
