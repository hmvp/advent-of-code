aoc::parts!(1, 2);

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut chars = parse_input(input.raw());

    let mut len = 0;
    while len != chars.len() {
        len = chars.len();
        chars = reduce(&chars);
    }
    len
}

fn reduce(chars: &[char]) -> Vec<char> {
    let mut chars = chars.iter();
    let mut o_prev_char = Some(*(chars.next().unwrap()));
    let mut new_chars = vec![];
    for next_char in chars {
        if let Some(prev_char) = o_prev_char {
            if (next_char.is_lowercase() && prev_char.is_uppercase() && next_char.to_ascii_uppercase() == prev_char)
                || (next_char.is_uppercase() && prev_char.is_lowercase() && next_char.to_ascii_lowercase() == prev_char)
            {
                o_prev_char = None;
                continue;
            }

            new_chars.push(prev_char);
        }
        o_prev_char = Some(*next_char);
    }
    if let Some(prev_char) = o_prev_char {
        new_chars.push(prev_char);
    }
    new_chars
}

fn part_2(input: aoc::Input) -> impl ToString {
    let chars = parse_input(input.raw());

    let mut shortest_len = 100_000;
    for letter in char_iter::new('a', 'z') {
        let mut chars = chars
            .iter()
            .filter(|i| **i != letter && **i != letter.to_ascii_uppercase())
            .copied()
            .collect::<Vec<char>>();

        let mut len = 0;
        while len != chars.len() {
            len = chars.len();
            chars = reduce(&chars);
        }
        if len < shortest_len {
            shortest_len = len;
        }
    }
    shortest_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce() {
        assert_eq!(reduce(&parse_input("a")), vec!['a']);
        assert_eq!(reduce(&parse_input("aA")), vec![]);
        assert_eq!(reduce(&parse_input("Aa")), vec![]);
        assert_eq!(reduce(&parse_input("Aac")), vec!['c']);
        assert_eq!(reduce(&parse_input("cAa")), vec!['c']);
        assert_eq!(reduce(&parse_input("cAac")), vec!['c', 'c']);
        assert_eq!(reduce(&parse_input("acA")), vec!['a', 'c', 'A']);
        assert_eq!(reduce(&parse_input("abBA")), vec!['a', 'A']);
        assert_eq!(reduce(&parse_input("aAbB")), vec![]);
    }
}
