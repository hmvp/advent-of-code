use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;


aoc::parts!(1, 2);


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Op {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, nr) = s.trim().split_at(3);
        let (sign, nr) = nr.split_at(1);
        let mut nr: i32 = nr.parse()?;

        if sign == "-" {
            nr = -nr;
        }

        Ok(match op {
            "jmp" => Op::Jmp(nr),
            "nop" => Op::Nop(nr),
            "acc" => Op::Acc(nr),
            _ => unreachable!(),
        })
    }
}

impl Op {
    fn execute(self, pc: &mut usize, acc: &mut isize) {
        match self {
            Op::Acc(nr) => {
                *pc += 1;
                *acc += nr as isize;
            }
            Op::Nop(_) => *pc += 1,
            Op::Jmp(nr) => {
                if nr >= 0 {
                    *pc += nr as usize
                } else {
                    *pc -= -nr as usize
                }
            }
        }
    }

    fn swap(&mut self) {
        match self {
            Op::Acc(_) => {}
            Op::Nop(nr) => *self = Op::Jmp(*nr),
            Op::Jmp(nr) => *self = Op::Nop(*nr),
        }
    }
}

pub fn input_generator(input: aoc::Input) -> Vec<Op> {
    input.lines().flat_map(str::parse).collect()
}

fn run(input: &[Op]) -> Result<isize, isize> {
    let mut pc = 0;
    let mut acc = 0;
    let mut seen: HashSet<usize> = HashSet::new();

    loop {
        if !seen.insert(pc) {
            return Err(acc);
        }

        if pc >= input.len() {
            return Ok(acc);
        }

        input[pc].execute(&mut pc, &mut acc);
    }
}



fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    run(input).unwrap_err()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input);

    let good_input = input
    .iter()
    .scan((input, 0), |(input, index), op| {
        *index += 1;
        if let Op::Acc(_) = op {
            Some(vec![])
        } else {
            let mut result = vec![input.to_vec(), input.to_vec()];
            result[0][*index - 1].swap();
            Some(result)
        }
    })
    .flatten()
    .find(|input| run(input).is_ok())
    .unwrap();

run(&good_input).unwrap()
}
