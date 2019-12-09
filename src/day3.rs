use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up(isize),
    Down(isize),
    Right(isize),
    Left(isize),
}

impl From<&str> for Direction {
    fn from(string: &str) -> Self {
        let (dir, length) = string.split_at(1);
        match dir {
            "U" => Direction::Up(length.parse().unwrap()),
            "D" => Direction::Down(length.parse().unwrap()),
            "R" => Direction::Right(length.parse().unwrap()),
            "L" => Direction::Left(length.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Direction>, Vec<Direction>) {
    let mut lines = input.lines();

    let f = |line: &str| {
        line.trim()
            .split(',')
            .map(|l| l.into())
            .collect::<Vec<Direction>>()
    };

    (f(lines.next().unwrap()), f(lines.next().unwrap()))
}

fn wire_coordinates(directions: &[Direction]) -> Vec<(isize, isize)> {
    let mut wire = vec![];
    let mut x: isize = 0;
    let mut y: isize = 0;

    for direction in directions {
        match *direction {
            Direction::Up(l) => {
                for _ in 0..l {
                    y += 1;
                    wire.push((x, y));
                }
            }
            Direction::Down(l) => {
                for _ in 0..l {
                    y -= 1;
                    wire.push((x, y));
                }
            }
            Direction::Right(l) => {
                for _ in 0..l {
                    x += 1;
                    wire.push((x, y));
                }
            }
            Direction::Left(l) => {
                for _ in 0..l {
                    x -= 1;
                    wire.push((x, y));
                }
            }
        }
    }

    wire
}

#[aoc(day3, part1)]
pub fn part1(input: &(Vec<Direction>, Vec<Direction>)) -> isize {
    let mut wire1 = wire_coordinates(&input.0);
    let wire2 = wire_coordinates(&input.1);

    wire1.retain(|c| wire2.contains(c));

    let mut crosses: Vec<isize> = wire1.iter().map(|(x, y)| x.abs() + y.abs()).collect();

    crosses.sort();
    crosses[0]
}

#[aoc(day3, part2)]
pub fn part2(input: &(Vec<Direction>, Vec<Direction>)) -> usize {
    let wire1 = wire_coordinates(&input.0);
    let wire2 = wire_coordinates(&input.1);

    let mut crosses: Vec<((isize, isize), usize)> = wire1
        .iter()
        .scan(0, |s, c| {
            *s += 1;
            Some((*c, *s))
        })
        .collect();

    crosses.retain(|(c, _s)| wire2.contains(c));

    let mut cross_values: Vec<usize> = crosses
        .iter()
        .map(|(c, s)| 1 + s + wire2.iter().position(|c2| c == c2).unwrap())
        .collect();

    cross_values.sort();
    cross_values[0]
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part2, wire_coordinates, Direction::*};

    #[test]
    fn check_input_generator() {
        assert_eq!(
            input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
                 U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            (
                vec![
                    Right(75),
                    Down(30),
                    Right(83),
                    Up(83),
                    Left(12),
                    Down(49),
                    Right(71),
                    Up(7),
                    Left(72)
                ],
                vec![
                    Up(62),
                    Right(66),
                    Up(55),
                    Right(34),
                    Down(71),
                    Right(55),
                    Down(58),
                    Right(83)
                ]
            )
        );
        assert_eq!(
            input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                 U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            (
                vec![
                    Right(98),
                    Up(47),
                    Right(26),
                    Down(63),
                    Right(33),
                    Up(87),
                    Left(62),
                    Down(20),
                    Right(33),
                    Up(53),
                    Right(51)
                ],
                vec![
                    Up(98),
                    Right(91),
                    Down(20),
                    Right(16),
                    Down(67),
                    Right(40),
                    Up(7),
                    Right(15),
                    Up(6),
                    Right(7)
                ]
            )
        );
    }

    #[test]
    fn check_wire_coordinates() {
        assert_eq!(
            wire_coordinates(&[Right(8), Up(5), Left(5), Down(3),]),
            vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (8, 5),
                (7, 5),
                (6, 5),
                (5, 5),
                (4, 5),
                (3, 5),
                (3, 4),
                (3, 3),
                (3, 2),
            ],
        );
        assert_eq!(
            wire_coordinates(&[Up(7), Right(6), Down(4), Left(4)]),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (0, 6),
                (0, 7),
                (1, 7),
                (2, 7),
                (3, 7),
                (4, 7),
                (5, 7),
                (6, 7),
                (6, 6),
                (6, 5),
                (6, 4),
                (6, 3),
                (5, 3),
                (4, 3),
                (3, 3),
                (2, 3)
            ]
        );
    }

    #[test]
    fn check_part2() {
        assert_eq!(
            part2(&input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
        assert_eq!(
            part2(&input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        )
    }
}
