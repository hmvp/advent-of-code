use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use trees::linked::{Node, Tree};
use trees::tr;

const ROOT: &str = "COM";

fn find_node_mut<'a, T: std::cmp::PartialEq>(
    node: &'a mut Node<T>,
    key: &T,
) -> Option<&'a mut Node<T>> {
    if node.data == *key {
        Some(node)
    } else {
        node.iter_mut().fold(None, |s, c| {
            if s.is_some() {
                s
            } else {
                find_node_mut(c, key)
            }
        })
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Tree<String> {
    let data: Vec<Vec<&str>> = input.lines().map(|l| l.split(')').collect()).collect();
    let mut map = HashMap::new();
    for pair in data {
        map.entry(pair[0]).or_insert_with(|| vec![]).push(pair[1]);
    }

    let mut tree = tr(ROOT.to_string());
    let mut queue = vec![ROOT];
    while !queue.is_empty() {
        let key = queue.pop().unwrap();
        let values = map.remove(key).unwrap_or_else(|| vec![]);

        let maybe_node = { find_node_mut(tree.root_mut(), &key.to_string()) };
        let node = if let Some(node) = maybe_node {
            node
        } else {
            tree.root_mut()
        };

        for value in values {
            node.push_front(tr(value.to_string()));
            queue.push(value);
        }
    }

    tree
}

fn count_to_root<T: std::fmt::Debug>(mut n: &Node<T>) -> u32 {
    let mut count = n.iter().map(count_to_root).sum::<u32>();

    while let Some(p) = n.parent() {
        n = p;
        count += 1;
    }
    count
}

#[aoc(day6, part1)]
fn part1(tree: &Tree<String>) -> u32 {
    count_to_root(tree.root())
}

fn find_node<'a, T: std::cmp::PartialEq>(node: &'a Node<T>, key: &T) -> Option<&'a Node<T>> {
    if node.data == *key {
        Some(node)
    } else {
        node.iter()
            .fold(None, |s, c| if s.is_some() { s } else { find_node(c, key) })
    }
}

fn path_to_root<T: std::fmt::Debug + std::cmp::PartialEq + Clone>(n: &Node<T>, key: &T) -> Vec<T> {
    let mut node = find_node(n, key);
    let mut result = Vec::new();

    while let Some(n) = node {
        node = n.parent();
        result.push(n.data.clone());
    }

    result
}

#[aoc(day6, part2)]
pub fn part2(tree: &Tree<String>) -> usize {
    let path_you = path_to_root(tree.root(), &"YOU".to_owned());
    let path_san = path_to_root(tree.root(), &"SAN".to_owned());

    let path_you_filtered = path_you
        .iter()
        .cloned()
        .take_while(|s| !path_san.contains(&s))
        .collect::<Vec<String>>();
    let path_san_filtered = path_san
        .iter()
        .cloned()
        .take_while(|s| !path_you.contains(&s))
        .collect::<Vec<String>>();

    path_san_filtered.len() + path_you_filtered.len() - 2
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2, tr};

    #[test]
    fn check_input_generator() {
        assert_eq!(
            input_generator(
                "COM)B\n\
                 B)C\n\
                 C)D\n\
                 D)E\n\
                 E)F\n\
                 B)G\n\
                 G)H\n\
                 D)I\n\
                 E)J\n\
                 J)K\n\
                 K)L"
            )
            .to_string(),
            "COM( B( G( H ) C( D( I E( J( K( L ) ) F ) ) ) ) )"
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(
            part1(
                &(tr("COM".to_owned())
                    / (tr("B".to_owned())
                        / (tr("G".to_owned()) / tr("H".to_owned()))
                        / (tr("C".to_owned())
                            / (tr("D".to_owned())
                                / (tr("E".to_owned())
                                    / (tr("J".to_owned())
                                        / (tr("K".to_owned()) / tr("L".to_owned())))
                                    / tr("F".to_owned()))
                                / tr("I".to_owned())))))
            ),
            42
        );
    }

    #[test]
    fn check_part2() {
        assert_eq!(
            part2(
                &(tr("COM".to_owned())
                    / (tr("B".to_owned())
                        / (tr("G".to_owned()) / tr("H".to_owned()))
                        / (tr("C".to_owned())
                            / (tr("D".to_owned())
                                / (tr("E".to_owned())
                                    / (tr("J".to_owned())
                                        / (tr("K".to_owned())
                                            / tr("L".to_owned())
                                            / tr("YOU".to_owned())))
                                    / tr("F".to_owned()))
                                / (tr("I".to_owned()) / tr("SAN".to_owned()))))))
            ),
            4
        );
    }
}
