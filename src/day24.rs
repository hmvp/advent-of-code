use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::collections::LinkedList;
use std::iter::FromIterator;

const HEIGHT: usize = 5;
const WIDTH: usize = 5;

#[derive(Clone, PartialEq)]
pub struct Map {
    grid: Vec<bool>,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = Ok(());
        result = result.and_then(|_| writeln!(f));
        for y in 0..HEIGHT {
            let offset = y * WIDTH;
            for x in 0..WIDTH {
                result = result
                    .and_then(|_| write!(f, "{}", if self.grid[offset + x] { "#" } else { "." }));
            }
            result = result.and_then(|_| writeln!(f));
        }
        result
    }
}

impl Map {
    pub fn next(&self) -> Self {
        let mut grid = self.grid.clone();
        for y in 0..HEIGHT {
            let offset = y * WIDTH;
            for x in 0..WIDTH {
                match (self.get_cell_state(x, y), self.neighbors_alive(x, y)) {
                    (true, 1) => grid[offset + x] = true,
                    (true, _) => grid[offset + x] = false,
                    (false, 1) | (false, 2) => grid[offset + x] = true,
                    _ => {}
                }
            }
        }
        Self { grid }
    }

    fn neighbors_alive(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;

        [(y, x - 1), (y - 1, x), (y + 1, x), (y, x + 1)]
            .iter()
            .filter(|&&(y, x)| !(x < 0 || x >= WIDTH as isize || y < 0 || y >= WIDTH as isize))
            .map(|&(y, x)| self.get_cell_state(x as usize, y as usize))
            .filter(|cell| *cell)
            .count()
    }

    fn get_cell_state(&self, x: usize, y: usize) -> bool {
        self.grid[(y * WIDTH + x)]
    }

    pub fn biodiversity(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, &x)| if x { 2usize.pow(i as u32) } else { 0 })
            .sum()
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Map {
    let grid: Vec<bool> = input
        .lines()
        .flat_map(|l| l.trim().chars().map(|c| c == '#'))
        .collect();

    Map { grid }
}

#[aoc(day24, part1)]
pub fn part1(input: &Map) -> usize {
    let mut map = input.clone();

    let mut seen = HashSet::new();

    loop {
        map = map.next();

        if !seen.insert(map.biodiversity()) {
            return map.biodiversity();
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecursiveMap {
    grids: LinkedList<Map>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            grid: vec![false; 5 * 5],
        }
    }
}

impl RecursiveMap {
    pub fn from(map: Map) -> Self {
        Self {
            grids: LinkedList::from_iter(Some(map)),
        }
    }

    pub fn next(&self) -> Self {
        let mut grids = self.grids.clone();

        if grids.front().unwrap().biodiversity() != 0 {
            grids.push_front(Map::default());
        }
        if grids.back().unwrap().biodiversity() != 0 {
            grids.push_back(Map::default());
        }

        // for i in 0 {
        //     // let prev = grids.

        //     let mut grid = map.unwrap();

        //     for y in 0..HEIGHT {
        //         let offset = y * WIDTH;
        //         for x in 0..WIDTH {
        //             if (x, y) == (2, 2) {
        //                 continue;
        //             }

        //             match (
        //                 grid.get_cell_state(x, y),
        //                 self.neighbors_alive(&prev, &grid, &next, x, y),
        //             ) {
        //                 (true, 1) => grid.grid[offset + x] = true,
        //                 (true, _) => grid.grid[offset + x] = false,
        //                 (false, 1) | (false, 2) => grid.grid[offset + x] = true,
        //                 _ => {}
        //             }
        //         }
        //     }
        //     map = Some(next);
        //     prev = &*grid;
        // }

        Self { grids }
    }

    fn neighbors_alive(&self, prev: &Map, this: &Map, next: &Map, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;

        let neighbors = [(y, x - 1), (y - 1, x), (y + 1, x), (y, x + 1)];

        neighbors
            .iter()
            .filter(|&&(y, x)| !(x < 0 || x >= WIDTH as isize || y < 0 || y >= WIDTH as isize))
            .filter(|&&(y, x)| !(y == 2 && x == 2))
            .map(|&(y, x)| this.get_cell_state(x as usize, y as usize))
            .filter(|cell| *cell)
            .count()
            + neighbors
                .iter()
                .filter(|&&(y, x)| x < 0 || x >= WIDTH as isize || y < 0 || y >= WIDTH as isize)
                .map(|&(y, x)| {
                    prev.get_cell_state(
                        2 + (x as usize % (WIDTH - 1)),
                        2 + (y as usize) % (HEIGHT - 1),
                    )
                })
                .filter(|cell| *cell)
                .count()
            + neighbors
                .iter()
                .filter(|&&(y, x)| y == 2 && x == 2)
                .map(|&(y, x)| {
                    prev.get_cell_state(
                        2 + (x as usize % (WIDTH - 1)),
                        2 + (y as usize) % (HEIGHT - 1),
                    )
                })
                .filter(|cell| *cell)
                .count()
    }
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::part1;

    #[test]
    fn check_next() {
        let map = input_generator(
            "....#
             #..#.
             #..##
             ..#..
             #....",
        );
        let map = map.next();
        assert_eq!(
            map,
            input_generator(
                "#..#.
                 ####.
                 ###.#
                 ##.##
                 .##.."
            )
        );
        let map = map.next();
        assert_eq!(
            map,
            input_generator(
                "#####
                ....#
                ....#
                ...#.
                #.###"
            )
        );
        let map = map.next();
        assert_eq!(
            map,
            input_generator(
                "#....
                ####.
                ...##
                #.##.
                .##.#
                "
            )
        );
    }

    #[test]
    fn check_neighbors_alive() {
        let map = input_generator(
            "....#
             #..#.
             #..##
             ..#..
             #....",
        );
        assert_eq!(map.neighbors_alive(0, 0), 1);
        assert_eq!(map.neighbors_alive(1, 0), 0);
        assert_eq!(map.neighbors_alive(0, 1), 1);
        assert_eq!(map.neighbors_alive(4, 1), 3);
    }

    #[test]
    fn check_get_cell_state() {
        let map = input_generator(
            "....#
             #..#.
             #..##
             ..#..
             #....",
        );
        assert_eq!(map.get_cell_state(0, 0), false);
        assert_eq!(map.get_cell_state(1, 0), false);
        assert_eq!(map.get_cell_state(0, 1), true);
        assert_eq!(map.get_cell_state(4, 1), false);
        assert_eq!(map.get_cell_state(4, 2), true);
    }
    #[test]
    fn check_biodiversity() {
        let map = input_generator(
            ".....
            .....
            .....
            #....
            .#...",
        );
        assert_eq!(map.biodiversity(), 2_129_920);
        let map = input_generator(
            ".....
            .....
            .....
            .....
            .....",
        );
        assert_eq!(map.biodiversity(), 0);
        let map = input_generator(
            "#####
            #####
            #####
            #####
            #####",
        );
        assert_eq!(map.biodiversity(), 33_554_431);
    }

    #[test]
    fn check_part1() {
        let map = input_generator(
            "....#
            #..#.
            #..##
            ..#..
            #....",
        );
        assert_eq!(part1(&map), 2_129_920);
    }
}
