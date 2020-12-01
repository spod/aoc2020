use std::io::BufRead;
pub trait Problem {
    fn new(&self, input: &dyn BufRead) -> &'static dyn Problem;
    fn part_1(&self) -> Result<i32, &str>;
    fn part_2(&self) -> Result<i32, &str>;
}
