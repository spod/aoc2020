use std::error::Error;
pub trait Problem {
    fn new(&self, inputfile: String) -> &'static dyn Problem;
    fn part_1(&self) -> Result<i32, Box<dyn Error>>;
    fn part_2(&self) -> Result<i32, Box<dyn Error>>;
}
