use itertools::Itertools;
use std::collections::HashMap;


aoc::parts!(1, 2);


#[derive(Debug, PartialEq)]
pub struct Image {
    id: usize,
    grid: Vec<Vec<bool>>,
}

impl Image {
    fn sides(&self) -> Vec<Vec<bool>> {
        let mut sides = self
            .grid
            .iter()
            .map(|row| (row[0], *row.last().unwrap()))
            .fold(vec![vec![], vec![]], |mut acc, r| {
                acc[0].push(r.0);
                acc[1].push(r.1);
                acc
            });
        sides.push(self.grid[0].clone());
        sides.push(self.grid.last().unwrap().clone());
        sides
    }
}

pub fn input_generator(input: &str) -> Vec<Image> {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let id = lines
                .by_ref()
                .next()
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .parse()
                .unwrap();
            let grid = lines
                .map(|l| l.chars().map(|c| c == '#').collect_vec())
                .collect();
            Image { id, grid }
        })
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut map = HashMap::new();

    let sides = input.iter().map(|i| (i, i.sides()));

    for (i, sides) in sides {
        assert_eq!(sides.len(), 4);
        for mut s in sides {
            map.entry(s.clone()).or_insert(vec![]).push(i);
            s.reverse();
            map.entry(s).or_insert(vec![]).push(i);
        }
    }

    dbg!(map.iter()
        .filter_map(|(_key, value)| {
            if value.len() == 1 {
                Some(value[0].id)
            } else {
                None
            }
        })
        .sorted().collect_vec()).iter()
        .batching(|it| {
            if let Some(start) = it.next() {
                dbg!(Some((start, it.take_while(|i| *i == start).count())))
            } else {
                None
            }
        })
        .filter_map(|(id, count)| if count >= 2 { Some(id) } else { None })
        .product::<usize>()
}


fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
    ";

    #[test]
    fn check_input_generator() {
        assert_eq!(
            &input_generator(
              INPUT
            )[0..1],
            &[Image {
                id: 2311,
                grid: vec![
                    vec![false, false, true, true, false, true, false, false, true, false],
                    vec![true, true, false, false, true, false, false, false, false, false],
                    vec![true, false, false, false, true, true, false, false, true, false],
                    vec![true, true, true, true, false, true, false, false, false, true],
                    vec![true, true, false, true, true, false, true, true, true, false],
                    vec![true, true, false, false, false, true, false, true, true, true],
                    vec![false, true, false, true, false, true, false, false, true, true],
                    vec![false, false, true, false, false, false, false, true, false, false],
                    vec![true, true, true, false, false, false, true, false, true, false],
                    vec![false, false, true, true, true, false, false, true, true, true]
                ]
            }]
        );
    }
}
