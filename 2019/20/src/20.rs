use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use std::collections::HashMap;

aoc::parts!(1);

type Loc = (usize, usize);

#[derive(Debug)]
pub struct Map {
    graph: UnGraph<Loc, usize>,
    begin: NodeIndex,
    end: NodeIndex,
}

pub fn input_generator(input: &str) -> Map {
    let mut locations = UnGraph::<Loc, usize>::default();
    let mut map = HashMap::<Loc, NodeIndex>::new();
    let mut begin = None;
    let mut end = None;

    let portal_letters = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_ascii_alphabetic() {
                    Some(((x, y), c))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<Loc, char>>();

    let mut portals = HashMap::<String, Vec<Loc>>::new();

    for (x, y) in input.lines().enumerate().flat_map(|(y, l)| {
        l.chars()
            .enumerate()
            .filter_map(move |(x, c)| if c == '.' { Some((x, y)) } else { None })
    }) {
        let node = locations.add_node((x, y));
        map.insert((x, y), node);
        map.get(&(x, y - 1)).map(|other| locations.add_edge(*other, node, 1));
        map.get(&(x - 1, y)).map(|other| locations.add_edge(*other, node, 1));
        portal_letters.get(&(x - 1, y)).map(|c| {
            portal_letters.get(&(x - 2, y)).map(|c2| {
                portals
                    .entry(c2.to_string() + &(*c).to_string())
                    .or_default()
                    .push((x, y));
            })
        });
        portal_letters.get(&(x + 1, y)).map(|c| {
            portal_letters.get(&(x + 2, y)).map(|c2| {
                portals
                    .entry(c.to_string() + &(*c2).to_string())
                    .or_default()
                    .push((x, y));
            })
        });
        portal_letters.get(&(x, y - 1)).map(|c| {
            portal_letters.get(&(x, y - 2)).map(|c2| {
                portals
                    .entry(c2.to_string() + &(*c).to_string())
                    .or_default()
                    .push((x, y));
            })
        });
        portal_letters.get(&(x, y + 1)).map(|c| {
            portal_letters.get(&(x, y + 2)).map(|c2| {
                portals
                    .entry(c.to_string() + &(*c2).to_string())
                    .or_default()
                    .push((x, y));
            })
        });
    }

    for (label, portal_locs) in portals {
        if label == "AA" {
            begin = Some(*map.get(&portal_locs[0]).unwrap());
        } else if label == "ZZ" {
            end = Some(*map.get(&portal_locs[0]).unwrap());
        } else if portal_locs.len() != 2 {
            dbg!(label, portal_locs);
            unreachable!();
        } else {
            map.get(&portal_locs[0])
                .map(|loc1| map.get(&portal_locs[1]).map(|loc2| locations.add_edge(*loc1, *loc2, 1)));
        }
    }
    locations.shrink_to_fit();

    if let (Some(begin), Some(end)) = (begin, end) {
        Map {
            graph: locations,
            begin,
            end,
        }
    } else {
        unimplemented!()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    astar(
        &input.graph,
        input.begin,
        |finish| finish == input.end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {

    //     #[test]
    //     fn check_part1() {
    //         assert_eq!(
    //             part1(&input_generator(
    //                 "
    //          A
    //          A
    //   #######.#########
    //   #######.........#
    //   #######.#######.#
    //   #######.#######.#
    //   #######.#######.#
    //   #####  B    ###.#
    // BC...##  C    ###.#
    //   ##.##       ###.#
    //   ##...DE  F  ###.#
    //   #####    G  ###.#
    //   #########.#####.#
    // DE..#######...###.#
    //   #.#########.###.#
    // FG..#########.....#
    //   ###########.#####
    //              Z
    //              Z       ",
    //             )),
    //             23
    //         );
    //     }
}
