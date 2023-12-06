use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many1};
use nom_supreme::final_parser::final_parser;
use std::collections::HashSet;

aoc::parts!(1, 2);

#[derive(Debug)]
pub enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Dir {
    fn do_move(&self, q: &mut isize, r: &mut isize) {
        match self {
            Dir::E => *q += 1,
            Dir::SE => *r += 1,
            Dir::SW => {
                *q -= 1;
                *r += 1;
            }
            Dir::W => *q -= 1,
            Dir::NW => *r -= 1,
            Dir::NE => {
                *q += 1;
                *r -= 1;
            }
        }
    }

    fn values() -> [Self; 6] {
        [Dir::E, Dir::SE, Dir::SW, Dir::W, Dir::NW, Dir::NE]
    }
}

fn directions(input: &str) -> Result<Vec<Dir>, ()> {
    let parser = many1::<_, _, (), _>(alt((
        map(tag("e"), |_| Dir::E),
        map(tag("se"), |_| Dir::SE),
        map(tag("sw"), |_| Dir::SW),
        map(tag("w"), |_| Dir::W),
        map(tag("nw"), |_| Dir::NW),
        map(tag("ne"), |_| Dir::NE),
    )));

    final_parser(parser)(input)
}

pub fn input_generator(input: &str) -> Vec<Vec<Dir>> {
    input.trim().lines().flat_map(directions).collect()
}

#[derive(Clone, PartialEq, Eq)]
pub struct Lobby {
    grid: HashSet<(isize, isize)>,
}

impl Lobby {
    pub fn from(input: &[Vec<Dir>]) -> Self {
        let mut flipped_tiles = HashSet::new();

        for i in input {
            let mut q = 0;
            let mut r = 0;

            for d in i {
                d.do_move(&mut q, &mut r);
            }

            if !flipped_tiles.remove(&(q, r)) {
                flipped_tiles.insert((q, r));
            }
        }
        Self { grid: flipped_tiles }
    }

    #[must_use]
    pub fn next(&self) -> Self {
        let mut grid = self.grid.clone();
        let q_start = *self.grid.iter().map(|(q, _)| q).min().unwrap() - 10;
        let q_end = *self.grid.iter().map(|(q, _)| q).max().unwrap() + 10;
        let r_start = *self.grid.iter().map(|(r, _)| r).min().unwrap() - 10;
        let r_end = *self.grid.iter().map(|(r, _)| r).max().unwrap() + 10;

        for q in q_start..=q_end {
            for r in r_start..=r_end {
                match (self.get_location_state(q, r), self.neighbor_black_tiles(q, r)) {
                    (true, 0) => {
                        grid.remove(&(q, r));
                    }
                    (true, x) if x > 2 => {
                        grid.remove(&(q, r));
                    }
                    (false, 2) => {
                        grid.insert((q, r));
                    }
                    _ => {}
                }
            }
        }
        Self { grid }
    }

    fn neighbor_black_tiles(&self, q: isize, r: isize) -> usize {
        Dir::values()
            .iter()
            .map(|dir| {
                let mut a = q;
                let mut b = r;
                dir.do_move(&mut a, &mut b);
                (a, b)
            })
            .map(|(q, r)| self.get_location_state(q, r))
            .filter(|i| *i)
            .count()
    }

    fn get_location_state(&self, q: isize, r: isize) -> bool {
        self.grid.contains(&(q, r))
    }

    pub fn black_tiles(&self) -> usize {
        self.grid.len()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    Lobby::from(input).black_tiles()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut lobby = Lobby::from(input);
    for _ in 0..100 {
        lobby = lobby.next();
        dbg!(lobby.black_tiles());
    }
    lobby.black_tiles()
}
