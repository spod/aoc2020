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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct ExecutionResult {
    terminated: bool,
    state: CPUState,
}

fn parse_instruction(instr: String) -> Instruction {
    let parts: Vec<&str> = instr.split(' ').collect();
    let op = match parts[0] {
        "acc" => Ops::ACC,
        "jmp" => Ops::JMP,
        "nop" => Ops::NOP,
        _ => unreachable!(),
    };
    let arg = parts[1].parse::<i32>().unwrap();
    Instruction { op: op, arg: arg }
}

// TODO - if this is re-used in later days wrap in Result<...> etc
fn run(mem: Vec<Instruction>) -> ExecutionResult {
    let mut state = CPUState { acc: 0, pc: 0 };
    let mut seen: Vec<bool> = vec![];
    let mut terminated: bool = true;
    for _ in 0..=mem.len() {
        seen.push(false);
    }

    loop {
        let i = &mem[state.pc as usize];
        if !seen[state.pc as usize] {
            seen[state.pc as usize] = true
        } else {
            // exit loop once we have "seen" every instruction in mem
            terminated = false;
            break;
        }
        match i.op {
            Ops::ACC => {
                state.acc += i.arg;
                state.pc += 1;
            }
            Ops::JMP => {
                state.pc += i.arg as isize;
            }
            Ops::NOP => {
                state.pc += 1;
            }
        };
    }
    ExecutionResult {
        terminated,
        state: state,
    }
}

fn part1(input: Vec<String>) -> i32 {
    let mut state = CPUState { acc: 0, pc: 0 };

    let mem: Vec<Instruction> = input.into_iter().map(|i| parse_instruction(i)).collect();

    let res = run(mem);

    println!("state.acc: {}", res.state.acc);
    res.state.acc
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

    #[test]
    fn day07_part1_real() {
        let input = load_input(false);
        assert_eq!(part1(input), 1528);
    }
}
