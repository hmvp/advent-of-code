aoc::parts!(1, 2);


fn make_row(x: usize) -> Vec<bool> {
    (0..x).map(|_| false).collect()
}

fn make_zplane(y: usize, x: usize) -> Vec<Vec<bool>> {
    let row = make_row(x);

    (0..y).map(|_| row.clone()).collect()
}

fn make_cube(z: usize, y: usize, x: usize) -> Vec<Vec<Vec<bool>>> {
    let cube = make_zplane(y, x);

    (0..z).map(|_| cube.clone()).collect()
}

#[derive(Clone, PartialEq, Eq)]
pub struct PocketDimension3D {
    grid: Vec<Vec<Vec<bool>>>,
}

impl std::fmt::Debug for PocketDimension3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = Ok(());
        result = result.and_then(|_| writeln!(f));
        for (z, z_planes) in self.grid.iter().enumerate() {
            result = result.and_then(|_| writeln!(f, "z={}", z));
            for row in z_planes {
                for x in row {
                    result = result.and_then(|_| write!(f, "{}", if *x { "#" } else { "." }));
                }
                result = result.and_then(|_| writeln!(f));
            }

            if z != self.grid.len() - 1 {
                result = result.and_then(|_| writeln!(f));
            }
        }
        result
    }
}

impl PocketDimension3D {
    pub fn next(&self) -> Self {
        let mut grid = self.grid.clone();
        grid.insert(0, make_zplane(grid[0].len(), grid[0][0].len()));
        grid.push(make_zplane(grid[0].len(), grid[0][0].len()));
        for (z, z_planes) in grid.iter_mut().enumerate() {
            z_planes.insert(0, make_row(z_planes[0].len()));
            z_planes.push(make_row(z_planes[0].len()));
            for (y, row) in z_planes.iter_mut().enumerate() {
                row.insert(0, false);
                row.push(false);
                for (x, cube) in row.iter_mut().enumerate() {
                    match (
                        &cube,
                        self.neighbor_cubes_active(x as isize - 1, y as isize - 1, z as isize - 1),
                    ) {
                        (false, 3) => {
                            *cube = true;
                        }
                        (true, cubes_active) if cubes_active != 2 && cubes_active != 3 => {
                            *cube = false
                        }
                        _ => {}
                    }
                }
            }
        }
        Self { grid }
    }

    fn neighbor_cubes_active(&self, x: isize, y: isize, z: isize) -> usize {
        [
            (z, y, x - 1),
            (z, y, x + 1),
            (z, y - 1, x - 1),
            (z, y - 1, x),
            (z, y - 1, x + 1),
            (z, y + 1, x - 1),
            (z, y + 1, x),
            (z, y + 1, x + 1),
            (z - 1, y, x),
            (z - 1, y, x - 1),
            (z - 1, y, x + 1),
            (z - 1, y - 1, x - 1),
            (z - 1, y - 1, x),
            (z - 1, y - 1, x + 1),
            (z - 1, y + 1, x - 1),
            (z - 1, y + 1, x),
            (z - 1, y + 1, x + 1),
            (z + 1, y, x),
            (z + 1, y, x - 1),
            (z + 1, y, x + 1),
            (z + 1, y - 1, x - 1),
            (z + 1, y - 1, x),
            (z + 1, y - 1, x + 1),
            (z + 1, y + 1, x - 1),
            (z + 1, y + 1, x),
            (z + 1, y + 1, x + 1),
        ]
        .iter()
        .filter(|&&(z, y, x)| {
            x >= 0
                && x < self.grid[0][0].len() as isize
                && y >= 0
                && y < self.grid[0].len() as isize
                && z >= 0
                && z < self.grid.len() as isize
        })
        .map(|&(z, y, x)| self.get_cube_state(x as usize, y as usize, z as usize))
        .filter(|i| *i)
        .count()
    }

    fn get_cube_state(&self, x: usize, y: usize, z: usize) -> bool {
        self.grid[z][y][x]
    }

    pub fn cubes_active(&self) -> usize {
        self.grid.iter().flatten().flatten().filter(|i| **i).count()
    }
}

pub fn input_generator(input: &str) -> PocketDimension3D {
    let input = input.trim();
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.trim().chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();

    PocketDimension3D { grid: vec![grid] }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PocketDimension4D {
    grid: Vec<Vec<Vec<Vec<bool>>>>,
}

impl std::fmt::Debug for PocketDimension4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = Ok(());
        result = result.and_then(|_| writeln!(f));
        for (w, cube) in self.grid.iter().enumerate() {
            for (z, z_planes) in cube.iter().enumerate() {
                result = result.and_then(|_| writeln!(f, "z={}, w={}", z, w));
                for row in z_planes {
                    for x in row {
                        result = result.and_then(|_| write!(f, "{}", if *x { "#" } else { "." }));
                    }
                    result = result.and_then(|_| writeln!(f));
                }

                if z != self.grid.len() - 1 {
                    result = result.and_then(|_| writeln!(f));
                }
            }
        }
        result
    }
}

impl PocketDimension4D {
    pub fn next(&self) -> Self {
        let mut grid = self.grid.clone();
        grid.insert(
            0,
            make_cube(grid[0].len(), grid[0][0].len(), grid[0][0][0].len()),
        );
        grid.push(make_cube(
            grid[0].len(),
            grid[0][0].len(),
            grid[0][0][0].len(),
        ));
        for (w, cube) in grid.iter_mut().enumerate() {
            cube.insert(0, make_zplane(cube[0].len(), cube[0][0].len()));
            cube.push(make_zplane(cube[0].len(), cube[0][0].len()));
            for (z, z_planes) in cube.iter_mut().enumerate() {
                z_planes.insert(0, make_row(z_planes[0].len()));
                z_planes.push(make_row(z_planes[0].len()));
                for (y, row) in z_planes.iter_mut().enumerate() {
                    row.insert(0, false);
                    row.push(false);
                    for (x, cube) in row.iter_mut().enumerate() {
                        match (
                            &cube,
                            self.neighbor_cubes_active(
                                x as isize - 1,
                                y as isize - 1,
                                z as isize - 1,
                                w as isize - 1,
                            ),
                        ) {
                            (false, 3) => {
                                *cube = true;
                            }
                            (true, cubes_active) if cubes_active != 2 && cubes_active != 3 => {
                                *cube = false
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Self { grid }
    }

    fn neighbor_cubes_active(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        [
            (w, z, y, x - 1),
            (w, z, y, x + 1),
            (w, z, y - 1, x - 1),
            (w, z, y - 1, x),
            (w, z, y - 1, x + 1),
            (w, z, y + 1, x - 1),
            (w, z, y + 1, x),
            (w, z, y + 1, x + 1),
            (w, z - 1, y, x),
            (w, z - 1, y, x - 1),
            (w, z - 1, y, x + 1),
            (w, z - 1, y - 1, x - 1),
            (w, z - 1, y - 1, x),
            (w, z - 1, y - 1, x + 1),
            (w, z - 1, y + 1, x - 1),
            (w, z - 1, y + 1, x),
            (w, z - 1, y + 1, x + 1),
            (w, z + 1, y, x),
            (w, z + 1, y, x - 1),
            (w, z + 1, y, x + 1),
            (w, z + 1, y - 1, x - 1),
            (w, z + 1, y - 1, x),
            (w, z + 1, y - 1, x + 1),
            (w, z + 1, y + 1, x - 1),
            (w, z + 1, y + 1, x),
            (w, z + 1, y + 1, x + 1),
            (w - 1, z, y, x),
            (w - 1, z, y, x - 1),
            (w - 1, z, y, x + 1),
            (w - 1, z, y - 1, x - 1),
            (w - 1, z, y - 1, x),
            (w - 1, z, y - 1, x + 1),
            (w - 1, z, y + 1, x - 1),
            (w - 1, z, y + 1, x),
            (w - 1, z, y + 1, x + 1),
            (w - 1, z - 1, y, x),
            (w - 1, z - 1, y, x - 1),
            (w - 1, z - 1, y, x + 1),
            (w - 1, z - 1, y - 1, x - 1),
            (w - 1, z - 1, y - 1, x),
            (w - 1, z - 1, y - 1, x + 1),
            (w - 1, z - 1, y + 1, x - 1),
            (w - 1, z - 1, y + 1, x),
            (w - 1, z - 1, y + 1, x + 1),
            (w - 1, z + 1, y, x),
            (w - 1, z + 1, y, x - 1),
            (w - 1, z + 1, y, x + 1),
            (w - 1, z + 1, y - 1, x - 1),
            (w - 1, z + 1, y - 1, x),
            (w - 1, z + 1, y - 1, x + 1),
            (w - 1, z + 1, y + 1, x - 1),
            (w - 1, z + 1, y + 1, x),
            (w - 1, z + 1, y + 1, x + 1),
            (w + 1, z, y, x),
            (w + 1, z, y, x - 1),
            (w + 1, z, y, x + 1),
            (w + 1, z, y - 1, x - 1),
            (w + 1, z, y - 1, x),
            (w + 1, z, y - 1, x + 1),
            (w + 1, z, y + 1, x - 1),
            (w + 1, z, y + 1, x),
            (w + 1, z, y + 1, x + 1),
            (w + 1, z - 1, y, x),
            (w + 1, z - 1, y, x - 1),
            (w + 1, z - 1, y, x + 1),
            (w + 1, z - 1, y - 1, x - 1),
            (w + 1, z - 1, y - 1, x),
            (w + 1, z - 1, y - 1, x + 1),
            (w + 1, z - 1, y + 1, x - 1),
            (w + 1, z - 1, y + 1, x),
            (w + 1, z - 1, y + 1, x + 1),
            (w + 1, z + 1, y, x),
            (w + 1, z + 1, y, x - 1),
            (w + 1, z + 1, y, x + 1),
            (w + 1, z + 1, y - 1, x - 1),
            (w + 1, z + 1, y - 1, x),
            (w + 1, z + 1, y - 1, x + 1),
            (w + 1, z + 1, y + 1, x - 1),
            (w + 1, z + 1, y + 1, x),
            (w + 1, z + 1, y + 1, x + 1),
        ]
        .iter()
        .filter(|&&(w, z, y, x)| {
            x >= 0
                && x < self.grid[0][0][0].len() as isize
                && y >= 0
                && y < self.grid[0][0].len() as isize
                && z >= 0
                && z < self.grid[0].len() as isize
                && w >= 0
                && w < self.grid.len() as isize
        })
        .map(|&(w, z, y, x)| self.get_cube_state(x as usize, y as usize, z as usize, w as usize))
        .filter(|i| *i)
        .count()
    }

    fn get_cube_state(&self, x: usize, y: usize, z: usize, w: usize) -> bool {
        self.grid[w][z][y][x]
    }

    pub fn cubes_active(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .filter(|i| **i)
            .count()
    }

    pub fn from(cube: &PocketDimension3D) -> Self {
        PocketDimension4D {grid: vec![cube.grid.clone()]}
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut input = input.clone();
    for _ in 0..6 {
        input = input.next();
    }

    input.cubes_active()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut input = PocketDimension4D::from(input);
    for _ in 0..6 {
        input = input.next();
    }

    input.cubes_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
.#.
..#
###
";

    #[test]
    fn check_input_generator() {
        let waiting_area = input_generator(INPUT);
        assert_eq!(
            format!("{:?}", waiting_area),
            "
z=0
.#.
..#
###
"
        );
    }

    #[test]
    fn check_neighbor_cubes_active() {
        let state1 = input_generator(INPUT);

        assert_eq!(state1.neighbor_cubes_active(0, 0, 0), 1);
        assert_eq!(state1.neighbor_cubes_active(0, 1, 0), 3);
        assert_eq!(state1.neighbor_cubes_active(1, 1, 0), 5);

        assert_eq!(state1.neighbor_cubes_active(0, 1, 1), 3);
        assert_eq!(state1.neighbor_cubes_active(2, 2, 1), 3);
        assert_eq!(state1.neighbor_cubes_active(1, 3, 1), 3);

        assert_eq!(state1.neighbor_cubes_active(0, 1, -1), 3);
        assert_eq!(state1.neighbor_cubes_active(2, 2, -1), 3);
        assert_eq!(state1.neighbor_cubes_active(1, 3, -1), 3);
    }

    #[test]
    fn check_get_cube_state() {
        let state1 = input_generator(INPUT);

        assert!(!state1.get_cube_state(0, 0, 0));
        assert!(state1.get_cube_state(0, 2, 0));
        assert!(state1.get_cube_state(2, 1, 0));
    }

    #[test]
    fn check_moves_part1() {
        let expected = "
z=0
.....
.....
.#...
...#.
..#..

z=1
.....
.....
.#.#.
..##.
..#..

z=2
.....
.....
.#...
...#.
..#..
";

        let waiting_area = input_generator(INPUT).next();

        assert_eq!(format!("{:?}", waiting_area), expected);

        let waiting_area = waiting_area.next();

        let expected = "
z=0
.......
.......
.......
.......
...#...
.......
.......

z=1
.......
.......
...#...
..#..#.
.....#.
..#....
.......

z=2
.......
.......
.##....
.##....
.#.....
.....#.
..###..

z=3
.......
.......
...#...
..#..#.
.....#.
..#....
.......

z=4
.......
.......
.......
.......
...#...
.......
.......
";

        assert_eq!(format!("{:?}", waiting_area), expected);
    }

    //     #[test]
    //     fn check_moves_part2() {
    //         let expected = "
    // #.##.##.##
    // #######.##
    // #.#.#..#..
    // ####.##.##
    // #.##.##.##
    // #.#####.##
    // ..#.#.....
    // ##########
    // #.######.#
    // #.#####.##
    // ";

    //         let waiting_area = input_generator(INPUT).next_part2();

    //         assert_eq!(format!("{:?}", waiting_area), expected);

    //         let waiting_area = waiting_area.next_part2();

    //         let expected = "
    // #.LL.LL.L#
    // #LLLLLL.LL
    // L.L.L..L..
    // LLLL.LL.LL
    // L.LL.LL.LL
    // L.LLLLL.LL
    // ..L.L.....
    // LLLLLLLLL#
    // #.LLLLLL.L
    // #.LLLLL.L#
    // ";

    //         assert_eq!(format!("{:?}", waiting_area), expected);

    //         let waiting_area = waiting_area.next_part2();

    //         let expected = "
    // #.L#.##.L#
    // #L#####.LL
    // L.#.#..#..
    // ##L#.##.##
    // #.##.#L.##
    // #.#####.#L
    // ..#.#.....
    // LLL####LL#
    // #.L#####.L
    // #.L####.L#
    // ";

    //         assert_eq!(format!("{:?}", waiting_area), expected);
    //     }

}
