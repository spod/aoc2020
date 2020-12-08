use prob::Problem;
use std::fs;
use std::io::BufRead;
pub struct Day08 {}

impl Problem for Day08 {
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

fn load_input(sample: bool) -> Vec<String> {
    let mut path = String::from("../inputs/day08");
    if sample {
        path.push_str("_sample");
    }
    let inlines = fs::read_to_string(path).unwrap();
    let result = inlines.lines().map(|s| String::from(s)).collect();
    result
}
struct CPUState {
    acc: i32,
    pc: isize,
}

#[derive(Debug, PartialEq)]
enum Ops {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    op: Ops,
    arg: i32,
}

fn parse_instruction(instr: String) -> Instruction {
    let mut parts: Vec<&str> = instr.split(' ').collect();
    let op = match parts[0] {
        "acc" => Ops::ACC,
        "jmp" => Ops::JMP,
        "nop" => Ops::NOP,
        _ => unreachable!(),
    };
    let arg = parts[1].parse::<i32>().unwrap();
    Instruction { op: op, arg: arg }
}

fn part1(input: Vec<String>) -> i32 {
    let mut state = CPUState { acc: 0, pc: 0 };

    let mem: Vec<Instruction> = input.into_iter().map(|i| parse_instruction(i)).collect();
    //    println!("mem: {:?}", mem);

    let mut c = 0;
    loop {
        c += 1;
        if c > 9000 {
            break;
        }
        let i = &mem[state.pc as usize];
        match i.op {
            Ops::ACC => {
                println!("Accumulate: {}", i.arg);
                state.acc += i.arg;
                state.pc += 1;
            }
            Ops::JMP => {
                println!("Jump: {}", i.arg);
                state.pc += i.arg as isize;
            }
            Ops::NOP => {
                println!("NOP: {}", i.arg);
                state.pc += 1;
            }
        };
    }
    5
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn day08_part1_sample() {
        let input = load_input(true);
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn day08_parse_instruction() {
        let tests = vec![
            (
                "nop +0",
                Instruction {
                    op: Ops::NOP,
                    arg: 0,
                },
            ),
            (
                "acc +1",
                Instruction {
                    op: Ops::ACC,
                    arg: 1,
                },
            ),
            (
                "jmp -3",
                Instruction {
                    op: Ops::JMP,
                    arg: -3,
                },
            ),
        ];
        for t in tests {
            assert_eq!(parse_instruction(t.0.to_string()), t.1);
        }
    }

    // #[test]
    // fn day07_part1_real() {
    //     let input = load_input(false);
    //     assert_eq!(part1(input), 4);
    // }
}
