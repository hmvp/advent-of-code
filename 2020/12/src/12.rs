use std::num::ParseIntError;
use std::str::FromStr;

aoc::parts!(1, 2);


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl Instruction {
    fn with_value(&self, n: usize) -> Self {
        use Instruction::*;
        match self {
            North(_) => North(n),
            South(_) => South(n),
            East(_) => East(n),
            West(_) => West(n),
            Left(_) => Left(n),
            Right(_) => Right(n),
            Forward(_) => Forward(n),
        }
    }

    fn turn(&self, n: isize) -> Self {
        use Instruction::*;

        let count = n / 90;
        let order = [North(0), East(0), South(0), West(0)];

        let start = match self {
            North(_) => 0,
            South(_) => 2,
            East(_) => 1,
            West(_) => 3,
            _ => unreachable!(),
        };

        order[(4 + start + count) as usize % 4]
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, n) = s.trim().split_at(1);
        let n: usize = n.parse()?;

        use Instruction::*;
        Ok(match op {
            "N" => North(n),
            "S" => South(n),
            "E" => East(n),
            "W" => West(n),
            "L" => Left(n),
            "R" => Right(n),
            "F" => Forward(n),
            _ => unreachable!(),
        })
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::East(0)
    }
}

pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(str::parse).collect()
}

#[derive(Default, Debug)]
struct Ship1 {
    loc_x: isize,
    loc_y: isize,
    heading: Instruction,
}

impl Ship1 {
    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            North(n) => self.loc_y += *n as isize,
            South(n) => self.loc_y -= *n as isize,
            East(n) => self.loc_x += *n as isize,
            West(n) => self.loc_x -= *n as isize,
            Left(n) => self.heading = self.heading.turn(-(*n as isize)),
            Right(n) => self.heading = self.heading.turn(*n as isize),
            Forward(n) => self.execute(&self.heading.with_value(*n)),
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.loc_x.abs() + self.loc_y.abs()) as usize
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    input
        .iter()
        .fold(Ship1::default(), |mut acc, i| {
            acc.execute(i);
            acc
        })
        .manhattan_distance()
}



#[derive(Debug)]
struct Ship2 {
    loc_x: isize,
    loc_y: isize,
    waypoint_x: isize,
    waypoint_y: isize,
}

impl Ship2 {
    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            North(n) => self.waypoint_y -= *n as isize,
            South(n) => self.waypoint_y += *n as isize,
            East(n) => self.waypoint_x += *n as isize,
            West(n) => self.waypoint_x -= *n as isize,
            Left(n) => match n {
                90 => {
                    let new_x = self.waypoint_y;
                    self.waypoint_y = -self.waypoint_x;
                    self.waypoint_x = new_x;
                }
                180 => {
                    let new_x = -self.waypoint_x;
                    self.waypoint_y = -self.waypoint_y;
                    self.waypoint_x = new_x;
                }
                270 => {
                    let new_x = self.waypoint_y;
                    self.waypoint_y = self.waypoint_x;
                    self.waypoint_x = -new_x;
                }
                _ => unimplemented!(),
            },
            Right(n) => match n {
                90 => {
                    let new_x = self.waypoint_y;
                    self.waypoint_y = self.waypoint_x;
                    self.waypoint_x = -new_x;
                }
                180 => {
                    let new_x = -self.waypoint_x;
                    self.waypoint_y = -self.waypoint_y;
                    self.waypoint_x = new_x;
                }
                270 => {
                    let new_x = self.waypoint_y;
                    self.waypoint_y = -self.waypoint_x;
                    self.waypoint_x = new_x;
                }
                _ => unimplemented!(),
            },
            Forward(n) => {
                self.loc_x += *n as isize * self.waypoint_x;
                self.loc_y += *n as isize * self.waypoint_y
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.loc_x.abs() + self.loc_y.abs()) as usize
    }
}

impl Default for Ship2 {
    fn default() -> Self {
        Ship2 {
            loc_x: 0,
            loc_y: 0,
            waypoint_x: 10,
            waypoint_y: -1,
        }
    }
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    input
        .iter()
        .fold(Ship2::default(), |mut acc, i| {
            acc.execute(i);
            acc
        })
        .manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_turn() {
        assert_eq!(Instruction::North(0).turn(90), Instruction::East(0));
        assert_eq!(Instruction::North(0).turn(-90), Instruction::West(0));
        assert_eq!(Instruction::South(0).turn(90), Instruction::West(0));
        assert_eq!(Instruction::South(0).turn(-90), Instruction::East(0));
        assert_eq!(Instruction::North(0).turn(180), Instruction::South(0));
        assert_eq!(Instruction::North(0).turn(-180), Instruction::South(0));
        assert_eq!(Instruction::South(0).turn(180), Instruction::North(0));
        assert_eq!(Instruction::South(0).turn(-180), Instruction::North(0));
    }
}
