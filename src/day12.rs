use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{tag, take, take_while1},
    combinator::map,
    combinator::opt,
    multi::separated_nonempty_list,
    sequence::delimited,
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use gcd::Gcd;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Moon {
    x: i16,
    y: i16,
    z: i16,
    vx: i8,
    vy: i8,
    vz: i8,
}

#[derive(Default)]
pub struct Gravity {
    x: i8,
    y: i8,
    z: i8,
}

impl Moon1D {
    pub fn calc_gravity(&self, other: &Self, gravity: &mut Gravity) {
        let vx = self.x.cmp(&other.x);

        match vx {
            Ordering::Greater => gravity.x -= 1,
            Ordering::Equal => {}
            Ordering::Less => gravity.x += 1,
        };
    }

    pub fn apply_gravity(&mut self, gravity: Gravity) {
        self.vx += gravity.x as i8;
    }

    pub fn apply_velocity(&mut self) {
        self.x += self.vx as i16;
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn coord(input: &str) -> IResult<&str, (&str, i16)> {
    let (rest, label) = take(1u8)(input)?;
    let (rest, _) = tag("=")(rest)?;
    let (rest, minus) = opt(tag("-"))(rest)?;
    let (rest, number) = take_while1(is_digit)(rest)?;

    let mut value: i16 = number.parse().unwrap();
    if minus.is_some() {
        value = -value;
    }
    IResult::Ok((rest, (label, value)))
}

fn parse_moon(input: &str) -> IResult<&str, Moon> {
    delimited(
        tag("<"),
        map(separated_nonempty_list(tag(", "), coord), |list| {
            let mut map: HashMap<&str, i16> = HashMap::from_iter(list.into_iter());
            Moon {
                x: map.remove(&"x").unwrap(),
                y: map.remove(&"y").unwrap(),
                z: map.remove(&"z").unwrap(),
                vx: 0,
                vy: 0,
                vz: 0,
            }
        }),
        tag(">"),
    )(input)
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Moon> {
    input.lines().map(|l| parse_moon(l).unwrap().1).collect()
}

pub fn iterate(times: usize, moons: &mut [Moon]) {
    let nr = moons.len();

    for _ in 0..times {
        for moona in 0..nr {
            let mut gravity = Gravity::default();
            for moonb in 0..nr {
                if moona != moonb {
                    moons[moona].calc_gravity(&moons[moonb], &mut gravity);
                }
            }
            moons[moona].apply_gravity(gravity);
        }
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(moons: &[Moon]) -> i16 {
    let mut moons = moons.to_vec();

    iterate(1000, &mut moons);

    moons.iter().map(|m| m.energy()).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Moon1D {
    x: i16,
    vx: i8,
}

impl Moon {
    pub fn energy(&self) -> i16 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs()) as i16
    }

    pub fn calc_gravity(&self, other: &Self, gravity: &mut Gravity) {
        let vx = self.x.cmp(&other.x);
        let vy = self.y.cmp(&other.y);
        let vz = self.z.cmp(&other.z);

        match vx {
            Ordering::Greater => gravity.x -= 1,
            Ordering::Equal => {}
            Ordering::Less => gravity.x += 1,
        };
        match vy {
            Ordering::Greater => gravity.y -= 1,
            Ordering::Equal => {}
            Ordering::Less => gravity.y += 1,
        };
        match vz {
            Ordering::Greater => gravity.z -= 1,
            Ordering::Equal => {}
            Ordering::Less => gravity.z += 1,
        };
    }

    pub fn apply_gravity(&mut self, gravity: Gravity) {
        self.vx += gravity.x as i8;
        self.vy += gravity.y as i8;
        self.vz += gravity.z as i8;
    }

    pub fn apply_velocity(&mut self) {
        self.x += self.vx as i16;
        self.y += self.vy as i16;
        self.z += self.vz as i16;
    }
}

pub fn iterate_part2(moons: &mut [Moon1D]) -> usize {
    let mut origs = HashSet::new();
    let nr = moons.len();
    let mut count = 0;
    origs.insert(moons.to_vec());

    loop {
        for moona in 0..nr {
            let mut gravity = Gravity::default();
            for moonb in 0..nr {
                if moona != moonb {
                    moons[moona].calc_gravity(&moons[moonb], &mut gravity);
                }
            }
            moons[moona].apply_gravity(gravity);
        }
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
        count += 1;

        if !origs.insert(moons.to_vec()) {
            break;
        }
    }
    count
}

#[aoc(day12, part2)]
pub fn part2(moons: &[Moon]) -> usize {
    let mut moonsx = moons
        .iter()
        .map(|m| Moon1D { x: m.x, vx: m.vx })
        .collect::<Vec<Moon1D>>();
    let x = iterate_part2(&mut moonsx);
    let mut moonsy = moons
        .iter()
        .map(|m| Moon1D { x: m.y, vx: m.vy })
        .collect::<Vec<Moon1D>>();
    let y = iterate_part2(&mut moonsy);
    let mut moonsz = moons
        .iter()
        .map(|m| Moon1D { x: m.z, vx: m.vz })
        .collect::<Vec<Moon1D>>();
    let z = iterate_part2(&mut moonsz);

    dbg!(x, y, z);
    let lcm_x_y = (x * y) / x.gcd(y);

    (lcm_x_y * z) / lcm_x_y.gcd(z)
}

#[cfg(test)]
mod tests {

    use super::{input_generator, iterate, Moon};

    #[test]
    fn check_input() {
        assert_eq!(
            input_generator("<x=13, y=-13, z=-2>\n<x=16, y=2, z=-15>"),
            vec![
                Moon {
                    x: 13,
                    y: -13,
                    z: -2,
                    vx: 0,
                    vy: 0,
                    vz: 0,
                },
                Moon {
                    x: 16,
                    y: 2,
                    z: -15,
                    vx: 0,
                    vy: 0,
                    vz: 0,
                },
            ]
        );
    }

    #[test]
    fn check_part1() {
        let moons = vec![
            Moon {
                x: 2,
                y: 1,
                z: -3,
                vx: -3,
                vy: -2,
                vz: 1,
            },
            Moon {
                x: 1,
                y: -8,
                z: 0,
                vx: -1,
                vy: 1,
                vz: 3,
            },
            Moon {
                x: 3,
                y: -6,
                z: 1,
                vx: 3,
                vy: 2,
                vz: -3,
            },
            Moon {
                x: 2,
                y: 0,
                z: 4,
                vx: 1,
                vy: -1,
                vz: -1,
            },
        ];

        assert_eq!(moons.iter().map(|m| m.energy()).sum::<i16>(), 179);

        let mut moons = input_generator(
            "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
        );

        assert_eq!(moons.iter().map(|m| m.energy()).sum::<i16>(), 0);

        iterate(1, &mut moons);

        assert_eq!(
            moons,
            vec![
                Moon {
                    x: 2,
                    y: -1,
                    z: 1,
                    vx: 3,
                    vy: -1,
                    vz: -1
                },
                Moon {
                    x: 3,
                    y: -7,
                    z: -4,
                    vx: 1,
                    vy: 3,
                    vz: 3
                },
                Moon {
                    x: 1,
                    y: -7,
                    z: 5,
                    vx: -3,
                    vy: 1,
                    vz: -3
                },
                Moon {
                    x: 2,
                    y: 2,
                    z: 0,
                    vx: -1,
                    vy: -3,
                    vz: 1
                }
            ]
        );

        assert_eq!(moons.iter().map(|m| m.energy()).sum::<i16>(), 229);

        iterate(9, &mut moons);

        assert_eq!(moons.iter().map(|m| m.energy()).sum::<i16>(), 179);

        let mut moons = input_generator(
            "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
        );

        iterate(100, &mut moons);

        assert_eq!(moons.iter().map(|m| m.energy()).sum::<i16>(), 1940);
    }
}
