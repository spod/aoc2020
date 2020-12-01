use std::error::Error;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.day {
        1 => println!("TODO run day 1 ..."),
        _ => eprintln!("Solution not implemented yet!"),
    }
    Ok(())
}
