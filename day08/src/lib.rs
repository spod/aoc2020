use prob::Problem;
use std::fs;
pub struct Day08 {}

impl Problem for Day08 {
    fn part_1(&self) -> Result<i32, &str> {
        let input = load_input(false);
        let res = part1(input);
        Ok(res)
    }

    fn part_2(&self) -> Result<i32, &str> {
        let input = load_input(false);
        let res = part2(input);
        Ok(res)
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Ops {
    ACC,
    JMP,
    NOP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
fn run(mem: &Vec<Instruction>) -> ExecutionResult {
    let mut state = CPUState { acc: 0, pc: 0 };
    let mut seen: Vec<bool> = vec![];
    let terminated: bool;
    for _ in 0..=mem.len() {
        seen.push(false);
    }

    loop {
        // check if we should exit execution loop
        // 1. execution terminates if pc goes past end of memory
        if state.pc as usize > mem.len() - 1 {
            terminated = true;
            break;
        }
        // 2. track mem accesses and assume infinite loop if we have accessed any instruction previously
        if !seen[state.pc as usize] {
            seen[state.pc as usize] = true
        } else {
            terminated = false;
            break;
        }

        // fetch instruction to execute and execute it
        let i = &mem[state.pc as usize];
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
        terminated: terminated,
        state: state,
    }
}

fn part1(input: Vec<String>) -> i32 {
    let mem: Vec<Instruction> = input.into_iter().map(|i| parse_instruction(i)).collect();

    let res = run(&mem);

    println!("state.acc: {}", res.state.acc);
    res.state.acc
}

fn swap(instr: Instruction) -> Instruction {
    // only swap jmp/nop instructions, leave acc untouched
    match instr.op {
        Ops::ACC => instr,
        Ops::JMP => Instruction {
            arg: instr.arg,
            op: Ops::NOP,
        },
        Ops::NOP => Instruction {
            arg: instr.arg,
            op: Ops::JMP,
        },
    }
}

fn part2(input: Vec<String>) -> i32 {
    let mut mem: Vec<Instruction> = input.into_iter().map(|i| parse_instruction(i)).collect();

    let mut res: ExecutionResult;

    for i in 0..=mem.len() - 1 {
        if mem[i].op == Ops::ACC {
            continue;
        }
        mem[i] = swap(mem[i]);
        res = run(&mem);
        if res.terminated {
            println!(
                "Terminated with instruction {} swapped; res: {:?}, mem:{:?}",
                i, res, mem
            );
            return res.state.acc;
        } else {
            // undo swap
            mem[i] = swap(mem[i]);
        }
    }
    -1
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn day08_part2_real() {
        let input = load_input(false);
        assert_eq!(part2(input), 640);
    }

    #[test]
    fn day08_part2_sample() {
        let input = load_input(true);
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn day08_test_swap() {
        let tests = vec![
            (
                parse_instruction(String::from("jmp -4")),
                Instruction {
                    op: Ops::NOP,
                    arg: -4,
                },
            ),
            (
                parse_instruction(String::from("nop 42")),
                Instruction {
                    op: Ops::JMP,
                    arg: 42,
                },
            ),
        ];
        for t in tests {
            assert_eq!(swap(t.0), t.1);
        }
    }
    #[test]
    fn day08_part1_sample() {
        let input = load_input(true);
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn day08_run_sample_loops() {
        let input = load_input(true);
        let mem: Vec<Instruction> = input.into_iter().map(|i| parse_instruction(i)).collect();
        let res = run(&mem);
        assert_eq!(res.terminated, false)
    }

    #[test]
    fn day08_run_sample2_terminates() {
        let inlines = fs::read_to_string("../inputs/day08_sample2").unwrap();
        let input: Vec<String> = inlines.lines().map(|s| String::from(s)).collect();
        let mem: Vec<Instruction> = input.into_iter().map(|i| parse_instruction(i)).collect();
        let res = run(&mem);
        assert_eq!(res.terminated, true)
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
    fn day08_part1_real() {
        let input = load_input(false);
        assert_eq!(part1(input), 1528);
    }
}
