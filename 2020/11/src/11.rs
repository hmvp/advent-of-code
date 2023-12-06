


aoc::parts!(1, 2);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Location {
    Taken,
    Empty,
    Floor,
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Location::Taken => write!(f, "#"),
            Location::Empty => write!(f, "L"),
            Location::Floor => write!(f, "."),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct WaitingArea {
    height: usize,
    width: usize,
    grid: Vec<Location>,
}

impl std::fmt::Debug for WaitingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = Ok(());
        result = result.and_then(|_| writeln!(f));
        for y in 0..self.height {
            let offset = y * self.width;
            for x in 0..self.width {
                result = result.and_then(|_| self.grid[offset + x].fmt(f));
            }
            result = result.and_then(|_| writeln!(f));
        }
        result
    }
}

impl WaitingArea {
    pub fn next_part1(&self) -> Self {
        let mut grid = self.grid.clone();
        for y in 0..self.height {
            let offset = y * self.width;
            for x in 0..self.width {
                let cell_state = self.grid[offset + x];
                if cell_state != Location::Floor {
                    match (cell_state, self.neighbor_seats_taken(x, y)) {
                        (Location::Empty, 0) => grid[offset + x] = Location::Taken,
                        (Location::Taken, seats_taken) if seats_taken >= 4 => {
                            grid[offset + x] = Location::Empty
                        }
                        _ => {}
                    }
                }
            }
        }
        Self {
            height: self.height,
            width: self.width,
            grid,
        }
    }

    pub fn next_part2(&self) -> Self {
        let mut grid = self.grid.clone();
        for y in 0..self.height {
            let offset = y * self.width;
            for x in 0..self.width {
                let cell_state = self.grid[offset + x];
                if cell_state != Location::Floor {
                    match (cell_state, self.neighbor_seats_taken_seen(x, y)) {
                        (Location::Empty, 0) => grid[offset + x] = Location::Taken,
                        (Location::Taken, seats_taken) if seats_taken >= 5 => {
                            grid[offset + x] = Location::Empty
                        }
                        _ => {}
                    }
                }
            }
        }
        Self {
            height: self.height,
            width: self.width,
            grid,
        }
    }

    fn neighbor_seats_taken(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;

        [
            (y, x - 1),
            (y, x + 1),
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y + 1, x - 1),
            (y + 1, x),
            (y + 1, x + 1),
        ]
        .iter()
        .filter(|&&(y, x)| x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize)
        .map(|&(y, x)| self.get_location_state(x as usize, y as usize))
        .filter(|i| *i == Location::Taken)
        .count()
    }

    fn neighbor_seats_taken_seen(&self, x: usize, y: usize) -> usize {
        [
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(|&(d_y, d_x)| {
            let mut x = d_x + x as isize;
            let mut y = d_y + y as isize;

            while x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {

                let state = self.get_location_state(x as usize, y as usize);
                if state == Location::Taken {
                    return Some(());
                } else if state == Location::Empty {
                    return None;
                }

                x += d_x;
                y += d_y;
            }
            None
        })
        .count()
    }

    fn get_location_state(&self, x: usize, y: usize) -> Location {
        self.grid[y * self.width + x]
    }

    pub fn seats_taken(&self) -> usize {
        self.grid.iter().filter(|i| **i == Location::Taken).count()
    }
}

pub fn input_generator(input: &str) -> WaitingArea {
    let input = input.trim();
    let width = input.lines().next().unwrap().len();
    let grid: Vec<Location> = input
        .lines()
        .flat_map(|l| {
            l.trim().chars().map(|c| {
                if c == 'L' {
                    Location::Empty
                } else if c == '#' {
                    Location::Taken
                } else {
                    Location::Floor
                }
            })
        })
        .collect();
    let height = grid.len() / width;

    WaitingArea {
        height,
        width,
        grid,
    }
}



fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut input = input.clone();
    loop {
        let new = input.next_part1();
        if new == input {
            break;
        }
        input = new;
    }

    input.seats_taken()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut input = input.clone();
    loop {
        let new = input.next_part2();
        if new == input {
            break;
        }
        input = new;
    }

    input.seats_taken()
}



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn check_input_generator() {
        let waiting_area = input_generator(INPUT);
        assert_eq!(format!("{:?}", waiting_area), INPUT);
        assert_eq!(waiting_area.width, 10);
        assert_eq!(waiting_area.height, 10);
    }

    #[test]
    fn check_neighbor_seats_taken() {
        let state1 = input_generator(INPUT);
        let state2 = state1.next_part1();
        let state3 = state2.next_part1();

        assert_eq!(state1.neighbor_seats_taken(0, 0), 0);
        assert_eq!(state2.neighbor_seats_taken(0, 0), 2);
        assert_eq!(state3.neighbor_seats_taken(0, 0), 1);

        assert_eq!(state1.neighbor_seats_taken(2, 0), 0);
        assert_eq!(state2.neighbor_seats_taken(2, 0), 4);
        assert_eq!(state3.neighbor_seats_taken(2, 0), 0);

        assert_eq!(state1.neighbor_seats_taken(4, 0), 0);
        assert_eq!(state2.neighbor_seats_taken(4, 0), 5);
        assert_eq!(state3.neighbor_seats_taken(4, 0), 0);
    }

    #[test]
    fn check_neighbor_seats_taken_seen() {
        let state1 = input_generator(
            "
.............
.L.L.#.#.#.#.
.............
",
        );
        assert_eq!(state1.neighbor_seats_taken_seen(0, 1), 0);

        assert_eq!(state1.neighbor_seats_taken_seen(1, 1), 0);

        assert_eq!(state1.neighbor_seats_taken_seen(3, 1), 1);
    }

    #[test]
    fn check_get_location_state() {
        let state1 = input_generator(INPUT);
        let state2 = state1.next_part1();
        let state3 = state2.next_part1();

        assert_eq!(state1.get_location_state(0, 0), Location::Empty);
        assert_eq!(state2.get_location_state(0, 0), Location::Taken);
        assert_eq!(state3.get_location_state(0, 0), Location::Taken);

        assert_eq!(state1.get_location_state(2, 0), Location::Empty);
        assert_eq!(state2.get_location_state(2, 0), Location::Taken);
        assert_eq!(state3.get_location_state(2, 0), Location::Empty);

        assert_eq!(state1.get_location_state(4, 0), Location::Floor);
        assert_eq!(state2.get_location_state(4, 0), Location::Floor);
        assert_eq!(state3.get_location_state(4, 0), Location::Floor);
    }

    #[test]
    fn check_moves_part1() {
        let expected = "
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";

        let waiting_area = input_generator(INPUT).next_part1();

        assert_eq!(format!("{:?}", waiting_area), expected);

        let waiting_area = waiting_area.next_part1();

        let expected = "
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
";

        assert_eq!(format!("{:?}", waiting_area), expected);

        let waiting_area = waiting_area.next_part1();

        let expected = "
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
";

        assert_eq!(format!("{:?}", waiting_area), expected);
    }

        #[test]
        fn check_moves_part2() {
            let expected = "
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";

            let waiting_area = input_generator(INPUT).next_part2();

            assert_eq!(format!("{:?}", waiting_area), expected);

            let waiting_area = waiting_area.next_part2();

            let expected = "
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
";

            assert_eq!(format!("{:?}", waiting_area), expected);

            let waiting_area = waiting_area.next_part2();

            let expected = "
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
";

            assert_eq!(format!("{:?}", waiting_area), expected);
        }
}
