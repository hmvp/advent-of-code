use std::collections::HashMap;

aoc::parts!(1, 2);

type Pos = (usize, usize);

#[derive(Debug)]
enum Pipe {
    Vertical(Option<usize>),
    Horizontal(Option<usize>),
    BendNE(Option<usize>),
    BendNW(Option<usize>),
    BendSW(Option<usize>),
    BendSE(Option<usize>),
    Empty,
    Start,
}

impl Pipe {
    fn from(c: char) -> Self {
        use Pipe::*;
        match c {
            '|' => Vertical(None),
            '-' => Horizontal(None),
            'L' => BendNE(None),
            'J' => BendNW(None),
            '7' => BendSW(None),
            'F' => BendSE(None),
            '.' => Empty,
            'S' => Start,
            _ => unreachable!(),
        }
    }

    fn set_count(&mut self, count: usize) {
        match self {
            Pipe::Vertical(Some(c))
            | Pipe::Horizontal(Some(c))
            | Pipe::BendNE(Some(c))
            | Pipe::BendNW(Some(c))
            | Pipe::BendSW(Some(c))
            | Pipe::BendSE(Some(c)) => *c = count,
            Pipe::Vertical(o @ None)
            | Pipe::Horizontal(o @ None)
            | Pipe::BendNE(o @ None)
            | Pipe::BendNW(o @ None)
            | Pipe::BendSW(o @ None)
            | Pipe::BendSE(o @ None) => *o = Some(count),
            Pipe::Empty | Pipe::Start => {}
        }
    }

    fn new_dir(&self, dir: &Dir) -> Dir {
        match (self, dir) {
            (Pipe::Vertical(_), Dir::North) | (Pipe::BendSW(_), Dir::West) | (Pipe::BendSE(_), Dir::East) => Dir::North,
            (Pipe::Vertical(_), Dir::South) | (Pipe::BendNE(_), Dir::East) | (Pipe::BendNW(_), Dir::West) => Dir::South,
            (Pipe::Horizontal(_), Dir::West) | (Pipe::BendNE(_), Dir::North) | (Pipe::BendSE(_), Dir::South) => {
                Dir::West
            }
            (Pipe::Horizontal(_), Dir::East) | (Pipe::BendNW(_), Dir::North) | (Pipe::BendSW(_), Dir::South) => {
                Dir::East
            }

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn new_pos(&self, pos: Pos) -> Pos {
        let (x, y) = pos;
        match self {
            Dir::North => (x, y + 1),
            Dir::South => (x, y - 1),
            Dir::West => (x + 1, y),
            Dir::East => (x - 1, y),
        }
    }
}

fn parse_input(input: aoc::Input) -> Vec<Vec<Pipe>> {
    input
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect()
}
#[allow(clippy::too_many_lines)]
fn find_path(grid: &mut HashMap<Pos, Pipe>) -> usize {
    let mut count = 0;

    let mut working_pos =
        grid.iter()
            .find(|(_, p)| matches!(p, Pipe::Start))
            .iter()
            .fold(vec![], |mut working_pos, ((x, y), _)| {
                let key = (x - 1, *y);
                let p = grid.get(&key).unwrap_or(&Pipe::Empty);
                if let Pipe::Horizontal(None) | Pipe::BendNE(None) | Pipe::BendSE(None) = p {
                    working_pos.push((key, Dir::East));
                }

                let key = (x + 1, *y);
                let p = grid.get(&key).unwrap_or(&Pipe::Empty);
                if let Pipe::Horizontal(None) | Pipe::BendNW(None) | Pipe::BendSW(None) = p {
                    working_pos.push((key, Dir::West));
                }

                let key = (*x, y - 1);
                let p = grid.get(&key).unwrap_or(&Pipe::Empty);
                if let Pipe::Vertical(None) | Pipe::BendNE(None) | Pipe::BendNW(None) = p {
                    working_pos.push((key, Dir::South));
                }

                let key = (*x, y + 1);
                let p = grid.get(&key).unwrap_or(&Pipe::Empty);
                if let Pipe::Vertical(None) | Pipe::BendSE(None) | Pipe::BendSW(None) = p {
                    working_pos.push((key, Dir::North));
                }
                working_pos
            });

    assert!(working_pos.len() == 2);

    'outer: loop {
        let mut new_working_pos = Vec::new();
        for (pos, dir) in &working_pos {
            let mut pipe = grid.get_mut(pos).unwrap();
            if let ref mut p @ (Pipe::Vertical(None)
            | Pipe::Horizontal(None)
            | Pipe::BendNE(None)
            | Pipe::BendNW(None)
            | Pipe::BendSW(None)
            | Pipe::BendSE(None)) = pipe
            {
                (*p).set_count(count);
                let dir = p.new_dir(dir);
                new_working_pos.push((dir.new_pos(*pos), dir));
            }
        }

        working_pos = new_working_pos;

        if working_pos.is_empty() {
            break 'outer;
        }

        count += 1;
    }

    // let width = grid.keys().map(|(x,_)|*x).max().unwrap();
    // let height = grid.keys().map(|(_,y)|*y).max().unwrap();
    // for py in 0..=height{
    //     for px in 0..=width {
    //         print!(
    //             "{}",
    //             grid.get(&((px), (py))).map_or(' ', |v| match v {
    //                 Pipe::Vertical(Some(_), _)
    //                 | Pipe::Horizontal(Some(_), _)
    //                 | Pipe::BendNE(Some(_), _)
    //                 | Pipe::BendNW(Some(_), _)
    //                 | Pipe::BendSW(Some(_), _)
    //                 | Pipe::BendSE(Some(_), _) => '#',
    //                 Pipe::Vertical(None, _) => '|',
    //                 Pipe::Horizontal(None, _) => '-',
    //                 Pipe::BendNE(None, _) => 'L',
    //                 Pipe::BendNW(None, _) => 'J',
    //                 Pipe::BendSW(None, _) => '7',
    //                 Pipe::BendSE(None, _) => 'F',
    //                 Pipe::Empty(_) => '.',
    //                 Pipe::Start => 'S',
    //             })
    //         );
    //     }
    //     println!();
    // }
    count
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut input = parse_input(input);

    let mut map = HashMap::new();

    for (y, mut line) in input.drain(..).enumerate() {
        for (x, pos) in line.drain(..).enumerate() {
            map.insert((x, y), pos);
        }
    }

    find_path(&mut map)
}

fn fill_outside(mut grid: HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
    'outer: loop {
        let mut found_empty = false;

        let mut new_grid = grid.clone();
        for ((x, y), value) in &grid {
            if value == &'O' {
                if grid.get(&(x - 1, *y)).unwrap_or(&'O') == &' ' {
                    found_empty = true;

                    new_grid.insert((x - 1, *y), 'O');
                }

                if grid.get(&(x + 1, *y)).unwrap_or(&'O') == &' ' {
                    found_empty = true;

                    new_grid.insert((x + 1, *y), 'O');
                }

                if grid.get(&(*x, y - 1)).unwrap_or(&'O') == &' ' {
                    found_empty = true;

                    new_grid.insert((*x, y - 1), 'O');
                }

                if grid.get(&(*x, y + 1)).unwrap_or(&'O') == &' ' {
                    found_empty = true;

                    new_grid.insert((*x, y + 1), 'O');
                }
            }
        }

        grid = new_grid;

        if !found_empty {
            break 'outer grid;
        }
    }
}
#[allow(clippy::too_many_lines)]
fn part_2(input: aoc::Input) -> impl ToString {
    let mut input = parse_input(input);

    let mut map = HashMap::new();

    for (y, mut line) in input.drain(..).enumerate() {
        for (x, pos) in line.drain(..).enumerate() {
            map.insert((x, y), pos);
        }
    }

    find_path(&mut map);

    // let width = map.keys().map(|(x, _)| *x).max().unwrap();
    // let height = map.keys().map(|(_, y)| *y).max().unwrap();

    // for py in 0..=height {
    //     for px in 0..=width {
    //         print!(
    //             "{}",
    //             map.get(&((px), (py))).map_or(' ', |v| match v {
    //                 Pipe::Vertical(Some(_))
    //                 | Pipe::Horizontal(Some(_))
    //                 | Pipe::BendNE(Some(_))
    //                 | Pipe::BendNW(Some(_))
    //                 | Pipe::BendSW(Some(_))
    //                 | Pipe::BendSE(Some(_)) => '#',
    //                 Pipe::Vertical(None) => '|',
    //                 Pipe::Horizontal(None) => '-',
    //                 Pipe::BendNE(None) => 'L',
    //                 Pipe::BendNW(None) => 'J',
    //                 Pipe::BendSW(None) => '7',
    //                 Pipe::BendSE(None) => 'F',
    //                 Pipe::Empty => '.',
    //                 Pipe::Start => 'S',
    //             })
    //         );
    //     }
    //     println!();
    // }

    let mut double_map = HashMap::new();

    for (&(mut x, mut y), pipe) in &map {
        x *= 2;
        y *= 2;
        x += 1;
        y += 1;
        let (a, b, c, d) = match pipe {
            Pipe::Vertical(Some(_)) | Pipe::BendSW(Some(_)) => ('#', ' ', '#', ' '),
            Pipe::Horizontal(Some(_)) | Pipe::BendNE(Some(_)) => ('#', '#', ' ', ' '),
            Pipe::BendNW(Some(_)) => ('#', ' ', ' ', ' '),
            Pipe::BendSE(Some(_)) => ('#', '#', '#', ' '),
            Pipe::Start => ('S', 'S', 'S', 'S'),
            _ => (' ', ' ', ' ', ' '),
        };
        double_map.insert((x, y), a);
        double_map.insert((x + 1, y), b);
        double_map.insert((x, y + 1), c);
        double_map.insert((x + 1, y + 1), d);
    }

    let double_width = double_map.keys().map(|(x, _)| *x).max().unwrap() + 1;
    let double_height = double_map.keys().map(|(_, y)| *y).max().unwrap() + 1;

    for x in 0..=double_width {
        double_map.insert((x, 0), 'O');
        double_map.insert((x, double_height), 'O');
    }

    for y in 0..=double_height {
        double_map.insert((0, y), 'O');
        double_map.insert((double_width, y), 'O');
    }

    double_map = fill_outside(double_map);

    // for py in 0..=double_height {
    //     for px in 0..=double_width {
    //         print!("{}", double_map.get(&((px), (py))).map_or(' ', |v| *v));
    //     }
    //     println!();
    // }

    let mut shrink_map = HashMap::with_capacity(map.len());

    for py in (1..double_height).step_by(2) {
        for px in (1..double_width).step_by(2) {
            if let (Some(' '), Some(' '), Some(' '), Some(' ')) = (
                double_map.get(&(px, py)),
                double_map.get(&(px + 1, py)),
                double_map.get(&(px, py + 1)),
                double_map.get(&(px + 1, py + 1)),
            ) {
                let mut x = px - 1;
                let mut y = py - 1;
                x /= 2;
                y /= 2;
                shrink_map.insert((x, y), 'I');
            }
        }
    }

    // for py in 0..=height {
    //     for px in 0..=width {
    //         print!("{}", shrink_map.get(&((px), (py))).map_or(' ', |v| *v));
    //     }
    //     println!();
    // }

    shrink_map.values().filter(|c| **c == 'I').count()
}
