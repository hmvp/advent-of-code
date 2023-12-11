use itertools::Itertools;
use std::collections::HashSet;

type Pos = (usize, usize);

aoc::parts!(1, 2);

struct Map {
    galaxies: Vec<Pos>,
    width: usize,
    heigth: usize,
}

impl Map {
    fn expand(&mut self, size: usize) {
        let rows = (0..self.heigth).collect::<HashSet<usize>>();
        let filled_rows = &self.galaxies.iter().map(|(_, y)| *y).collect::<HashSet<usize>>();
        let empty_rows: Vec<_> = rows.difference(filled_rows).collect();

        let columns = (0..self.width).collect::<HashSet<usize>>();
        let filled_columns = &self.galaxies.iter().map(|(x, _)| *x).collect::<HashSet<usize>>();
        let empty_columns: Vec<_> = columns.difference(filled_columns).collect();

        for (x, y) in &mut self.galaxies {
            *x += empty_columns.iter().filter(|i| **i < x).count() * (size - 1);
            *y += empty_rows.iter().filter(|i| **i < y).count() * (size - 1);
        }

        self.heigth += empty_rows.len() * (size - 1);
        self.width += empty_columns.len() * (size - 1);
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.heigth {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.galaxies.iter().any(|(gx, gy)| &x == gx && &y == gy) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_input(input: aoc::Input) -> Map {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect();

    Map {
        galaxies,
        width: input.lines().next().unwrap().len(),
        heigth: input.lines().count(),
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut map = parse_input(input);

    // println!("{map}");

    map.expand(2);

    // println!("{map}");

    map.galaxies
        .iter()
        .permutations(2)
        .map(|perms| {
            if let [a, b] = perms[..] {
                a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
            } else {
                unreachable!()
            }
        })
        .sum::<usize>()
        / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut map = parse_input(input);

    map.expand(1_000_000);

    map.galaxies
        .iter()
        .permutations(2)
        .map(|perms| {
            if let [a, b] = perms[..] {
                a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
            } else {
                unreachable!()
            }
        })
        .sum::<usize>()
        / 2
}
