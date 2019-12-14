use aoc_runner_derive::{aoc, aoc_generator};
use gcd::Gcd;
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Astroid {
    x: isize,
    y: isize,
}

impl PartialOrd for Astroid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Astroid {
    fn cmp(&self, other: &Self) -> Ordering {
        let atan = (self.x as f64).atan2(self.y as f64);
        let btan = (other.x as f64).atan2(other.y as f64);

        let arc = btan.partial_cmp(&atan).unwrap();

        if arc == Ordering::Equal {
            self.man_distance().cmp(&other.man_distance())
        } else {
            arc
        }
    }
}

impl Astroid {
    pub fn relative_coordinates(&self, other: &Astroid) -> Astroid {
        Astroid {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn man_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}
#[derive(Debug, PartialEq)]
pub struct Map {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    astroids: BTreeSet<Astroid>,
}

impl Map {
    pub fn is_within(&self, astroid: &Astroid) -> bool {
        astroid.x >= self.x
            && astroid.x < (self.width + self.x)
            && astroid.y >= self.y
            && astroid.y < (self.height + self.y)
    }

    pub fn filter_invisible(&self, astroid: &Astroid) -> Map {
        let mut map = Map {
            x: -astroid.x,
            y: -astroid.y,
            width: self.width,
            height: self.height,
            astroids: self
                .astroids
                .iter()
                .filter(|&a| a != astroid)
                .map(|a| astroid.relative_coordinates(a))
                .collect(),
        };
        map.clear_projections();
        map
    }

    pub fn projections(&self, astroid: &Astroid) -> Vec<Astroid> {
        if astroid.x == 0 && astroid.y == 0 {
            return vec![];
        }

        let gcd = (astroid.x.abs() as usize).gcd(astroid.y.abs() as usize) as isize;

        let a = Astroid {
            x: astroid.x / gcd,
            y: astroid.y / gcd,
        };

        (0..self.width)
            .map(|i| Astroid {
                x: a.x * i,
                y: a.y * i,
            })
            .skip_while(|a| a.man_distance() <= astroid.man_distance())
            .take_while(|a| self.is_within(a))
            .collect()
    }

    pub fn clear_projections(&mut self) {
        let p = self
            .astroids
            .iter()
            .flat_map(|a| self.projections(a))
            .collect::<Vec<Astroid>>();
        for a in p {
            self.astroids.remove(&a);
        }
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    let astroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Astroid {
                            x: x as isize,
                            y: y as isize,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Astroid>>()
        })
        .collect();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    Map {
        x: 0,
        y: 0,
        width: width as isize,
        height: height as isize,
        astroids,
    }
}

fn number_of_astroids(coordinate: &Astroid, map: &Map) -> usize {
    map.filter_invisible(coordinate).astroids.len()
}

#[aoc(day10, part1)]
fn part1(map: &Map) -> usize {
    dbg!(map
        .astroids
        .iter()
        .map(|a| (number_of_astroids(a, map), a))
        .max_by_key(|&(c, _)| c)
        .unwrap())
    .0
}

#[aoc(day10, part2)]
fn part2(map: &Map) -> isize {
    let station = Astroid { x: 22, y: 25 };
    let map = map.filter_invisible(&station);

    let n200 = dbg!(&map.astroids).iter().nth(199).unwrap();

    (n200.x + station.x) * 100 + (n200.y + station.y)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, Astroid, Map};
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    #[test]
    fn check_input_generator() {
        assert_eq!(
            input_generator(".#..#\n.....\n#####\n....#\n...##"),
            Map {
                x: 0,
                y: 0,
                width: 5,
                height: 5,
                astroids: BTreeSet::from_iter(
                    vec![
                        Astroid { x: 1, y: 0 },
                        Astroid { x: 4, y: 0 },
                        Astroid { x: 0, y: 2 },
                        Astroid { x: 1, y: 2 },
                        Astroid { x: 2, y: 2 },
                        Astroid { x: 3, y: 2 },
                        Astroid { x: 4, y: 2 },
                        Astroid { x: 4, y: 3 },
                        Astroid { x: 3, y: 4 },
                        Astroid { x: 4, y: 4 }
                    ]
                    .into_iter()
                )
            }
        );
    }

    #[test]
    fn check_projections() {
        let map = input_generator(".#..#\n.....\n#####\n....#\n...##");
        assert_eq!(
            map.projections(&Astroid { x: 1, y: 0 }),
            vec![
                Astroid { x: 2, y: 0 },
                Astroid { x: 3, y: 0 },
                Astroid { x: 4, y: 0 },
            ]
        );
        assert_eq!(
            map.projections(&Astroid { x: 1, y: 1 }),
            vec![
                Astroid { x: 2, y: 2 },
                Astroid { x: 3, y: 3 },
                Astroid { x: 4, y: 4 },
            ]
        );
        assert_eq!(
            map.projections(&Astroid { x: 1, y: 2 }),
            vec![Astroid { x: 2, y: 4 },]
        );

        let map = input_generator(".#..#\n.....\n#####\n....#\n...##")
            .filter_invisible(&Astroid { x: 2, y: 2 });

        assert_eq!(map.projections(&Astroid { x: 1, y: 2 }), vec![]);
        assert_eq!(
            map.projections(&Astroid { x: 1, y: 1 }),
            vec![Astroid { x: 2, y: 2 }]
        );
        assert_eq!(map.projections(&Astroid { x: -1, y: -2 }), vec![]);
        assert_eq!(
            map.projections(&Astroid { x: -1, y: 1 }),
            vec![Astroid { x: -2, y: 2 },]
        );
    }

    #[test]
    fn check_clear_projections() {
        let map = input_generator(".#..#\n.....\n#####\n....#\n...##")
            .filter_invisible(&Astroid { x: 4, y: 2 });
        assert_eq!(
            map.astroids,
            BTreeSet::from_iter(
                vec![
                    Astroid { x: 0, y: -2 },
                    Astroid { x: 0, y: 1 },
                    Astroid { x: -1, y: 2 },
                    Astroid { x: -1, y: 0 },
                    Astroid { x: -3, y: -2 }
                ]
                .into_iter()
            )
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(part1(&input_generator("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")), 33);
        assert_eq!(
            part1(&input_generator("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###."
            )),
            35
        );
        assert_eq!(part1(&input_generator(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..")), 41);
    }

    #[test]
    fn check_part2() {
        let map = input_generator(".###.\n##.##\n.###.\n##.##\n.###.");

        let station = Astroid { x: 2, y: 2 };
        let map = map.filter_invisible(&station);

        assert_eq!(
            map.astroids.into_iter().collect::<Vec<Astroid>>(),
            vec![
                Astroid { x: 0, y: -2 },
                Astroid { x: 1, y: -2 },
                Astroid { x: 1, y: -1 },
                Astroid { x: 2, y: -1 },
                Astroid { x: 1, y: 0 },
                Astroid { x: 2, y: 1 },
                Astroid { x: 1, y: 1 },
                Astroid { x: 1, y: 2 },
                Astroid { x: 0, y: 2 },
                Astroid { x: -1, y: 2 },
                Astroid { x: -1, y: 1 },
                Astroid { x: -2, y: 1 },
                Astroid { x: -1, y: 0 },
                Astroid { x: -2, y: -1 },
                Astroid { x: -1, y: -1 },
                Astroid { x: -1, y: -2 }
            ]
        );
    }
}
