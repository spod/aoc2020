pub trait Problem {
    fn part_1(&self) -> Result<i32, &str>;
    fn part_2(&self) -> Result<i32, &str>;
}
