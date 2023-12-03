use aoc::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated};

aoc::parts!(1, 2);

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn new() -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn red(red: usize) -> Self {
        CubeSet {
            red,
            green: 0,
            blue: 0,
        }
    }

    fn green(green: usize) -> Self {
        CubeSet {
            red: 0,
            green,
            blue: 0,
        }
    }

    fn blue(blue: usize) -> Self {
        CubeSet {
            red: 0,
            green: 0,
            blue,
        }
    }
    
    #[allow(clippy::needless_pass_by_value)]
    fn update(mut self, other: Self) -> Self {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
        self
    }

    fn fit(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

struct Game {
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn max_counts(&self) -> CubeSet {
        self.cube_sets
            .iter()
            .fold(CubeSet::new(), |mut acc, cube_set| {
                if acc.red < cube_set.red {
                    acc.red = cube_set.red;
                }
                if acc.green < cube_set.green {
                    acc.green = cube_set.green;
                }
                if acc.blue < cube_set.blue {
                    acc.blue = cube_set.blue;
                }
                acc
            })
    }

    fn fit(&self, other: &CubeSet) -> bool {
        self.cube_sets.iter().fold(
            true,
            |acc, set| if set.fit(other) { acc } else { false },
        )
    }
}

fn parse_input(input: Input) -> Vec<Game> {
    let color_value = alt((
        map(terminated(digit1::<&str, ()>, tag(" red")), |d: &str| {
            CubeSet::red(d.parse().unwrap())
        }),
        map(terminated(digit1::<&str, ()>, tag(" green")), |d: &str| {
            CubeSet::green(d.parse().unwrap())
        }),
        map(terminated(digit1::<&str, ()>, tag(" blue")), |d: &str| {
            CubeSet::blue(d.parse().unwrap())
        }),
    ));

    let set_parser = map(separated_list1(tag(", "), color_value), |mut sets| {
        sets.drain(..).reduce(CubeSet::update).unwrap()
    });
    let mut game_parser = map(
        preceded(
            delimited(tag("Game "), digit1, tag(": ")),
            separated_list1(tag("; "), set_parser),
        ),
        |cube_sets| Game { cube_sets },
    );

    input.lines().fold(Vec::new(), |mut acc, item| {
        acc.push(game_parser(item).unwrap().1);
        acc
    })
}

fn part_1(input: Input) -> impl ToString {
    let start = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    parse_input(input)
        .drain(..)
        .enumerate()
        .fold(0, |acc, (index, game)| {
            if game.fit(&start) {
                acc + 1 + index
            } else {
                acc
            }
        })
}

fn part_2(input: Input) -> impl ToString {
    parse_input(input)
        .drain(..)
        .map(|game| game.max_counts().power())
        .sum::<usize>()
}
