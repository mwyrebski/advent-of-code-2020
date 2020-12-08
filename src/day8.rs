mod code_runner {
    use crate::lib::split_once;

    #[derive(Copy, Clone)]
    enum Op {
        Nop(i32),
        Acc(i32),
        Jmp(i32),
    }

    pub struct BootCode {
        ops: Vec<Op>,
    }

    enum ExitCode {
        Success(i32),
        LoopDetected(i32),
    }

    impl ExitCode {
        pub fn value(&self) -> i32 {
            return match self {
                ExitCode::Success(x) => *x,
                ExitCode::LoopDetected(x) => *x,
            };
        }
    }

    impl BootCode {
        fn do_run<F>(len: usize, get: F) -> ExitCode
        where
            F: Fn(usize) -> Op,
        {
            let mut visited = vec![false; len];
            let mut eax = 0;
            let mut eip = 0;
            loop {
                if eip >= len {
                    return ExitCode::Success(eax);
                }
                if visited[eip] {
                    return ExitCode::LoopDetected(eax);
                }
                visited[eip] = true;
                match get(eip) {
                    Op::Nop(_) => eip += 1,
                    Op::Acc(x) => {
                        eip += 1;
                        eax += x
                    }
                    Op::Jmp(x) => eip = (eip as i32 + x) as usize,
                }
            }
        }

        pub fn run(&self) -> i32 {
            BootCode::do_run(self.ops.len(), |index| self.ops[index]).value()
        }

        pub fn run_with_fixing(&self) -> i32 {
            for fix_index in 0..self.ops.len() {
                let get_op = |index| {
                    let op = &self.ops[index];
                    if index == fix_index {
                        match op {
                            Op::Nop(x) => Op::Jmp(*x),
                            Op::Jmp(x) => Op::Nop(*x),
                            _ => *op,
                        }
                    } else {
                        *op
                    }
                };

                if let ExitCode::Success(exit) = BootCode::do_run(self.ops.len(), get_op) {
                    return exit;
                }
            }
            -1
        }
    }

    pub fn parse_input<'a>(lines: impl Iterator<Item = &'a str>) -> BootCode {
        let v = lines
            .map(|line| {
                let (o, p) = split_once(line, ' ').unwrap();
                let i = p.parse::<i32>().unwrap();

                match o {
                    "acc" => Op::Acc(i),
                    "jmp" => Op::Jmp(i),
                    "nop" => Op::Nop(i),
                    _ => panic!(),
                }
            })
            .collect::<Vec<_>>();
        BootCode { ops: v }
    }
}

use code_runner::{parse_input, BootCode};

fn part1(code: &BootCode) -> i32 {
    code.run()
}

fn part2(code: &BootCode) -> i32 {
    code.run_with_fixing()
}

pub fn run() {
    let input = include_str!("input/day8.txt");
    let code = &parse_input(input.lines());
    println!("Day 8/1: {}", part1(code));
    println!("Day 8/2: {}", part2(code));
}

#[cfg(test)]
mod tests {
    use super::code_runner::*;
    use super::*;

    const SAMPLE: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn test_part1_sample() {
        let code = parse_input(SAMPLE.lines());
        assert_eq!(5, part1(&code));
    }
    #[test]
    fn test_part2_sample() {
        let code = parse_input(SAMPLE.lines());
        assert_eq!(8, part2(&code));
    }
}
