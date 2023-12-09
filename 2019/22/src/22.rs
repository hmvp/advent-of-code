use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::sequence::preceded;
use nom::IResult;

aoc::parts!(1, 2);

#[derive(Clone, Debug)]
pub enum Deal {
    IntoNewStack,
    Cut(usize),
    CutMinus(usize),
    WithIncrements(usize),
}

impl Deal {
    fn execute(&self, stack: &mut [usize]) {
        match *self {
            Deal::IntoNewStack => stack.reverse(),
            Deal::Cut(i) => stack.rotate_left(i),
            Deal::CutMinus(i) => stack.rotate_right(i),
            Deal::WithIncrements(i) => {
                let mut temp = vec![0; stack.len()];
                temp.clone_from_slice(stack);
                for (n, x) in temp.drain(..).enumerate() {
                    let pos = n * i % stack.len();
                    stack[pos] = x
                }
            }
        }
    }

    fn calc(&self, stack_len: usize, n: usize) -> usize {
        match *self {
            Deal::IntoNewStack => stack_len - n - 1,
            Deal::Cut(i) => (n + i) % stack_len,
            Deal::CutMinus(i) => (n - i) % stack_len,
            Deal::WithIncrements(i) => n / i + (stack_len % i) * n,
        }
    }
}

fn deal(input: &str) -> IResult<&str, Deal> {
    let cut = map_res(preceded(tag("cut "), digit1), |n: &str| n.parse().map(Deal::Cut));
    let cut_minus = map_res(preceded(tag("cut -"), digit1), |n: &str| n.parse().map(Deal::CutMinus));
    let with_increments = map_res(preceded(tag("deal with increment "), digit1), |n: &str| {
        n.parse().map(Deal::WithIncrements)
    });
    let into_new_stack = value(Deal::IntoNewStack, tag("deal into new stack"));

    alt((cut, cut_minus, with_increments, into_new_stack))(input)
}

pub fn input_generator(input: &str) -> Vec<Deal> {
    input.lines().flat_map(|l| deal(l.trim()).map(|(_, d)| d)).collect()
}

pub fn shuffle(input: &[Deal], cards: &mut [usize]) {
    for deal in input {
        deal.execute(cards)
    }
}

pub fn calc_shuffle(input: &[Deal], stack_len: usize, n: usize) -> usize {
    let mut result = n;
    for deal in input {
        result = dbg!(dbg!(deal).calc(stack_len, dbg!(result)));
    }
    result
}

fn part_1(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut cards: Vec<usize> = (0..10_007).collect();

    shuffle(input, &mut cards);

    cards.iter().position(|&x| x == 2019).unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = &input_generator(input.raw());

    let mut result = 2020;
    for i in 0..101_741_582_076_661_usize {
        result = calc_shuffle(input, 119_315_717_514_047, result);

        if result == 2020 {
            dbg!(&i);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::calc_shuffle;
    use super::input_generator;
    use super::shuffle;

    #[test]
    fn check_part1() {
        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(&input_generator(""), &mut cards);
        assert_eq!(cards, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(
            &input_generator(
                "deal with increment 7
                deal into new stack
                deal into new stack",
            ),
            &mut cards,
        );
        assert_eq!(cards, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let mut cards: Vec<usize> = (0..11).collect();
        shuffle(
            &input_generator(
                "deal with increment 7
                deal into new stack
                deal into new stack",
            ),
            &mut cards,
        );
        assert_eq!(cards, [0, 8, 5, 2, 10, 7, 4, 1, 9, 6, 3]);

        let mut cards: Vec<usize> = (0..12).collect();
        shuffle(
            &input_generator(
                "deal with increment 7
                deal into new stack
                deal into new stack",
            ),
            &mut cards,
        );
        assert_eq!(cards, [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5]);

        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(
            &input_generator(
                "cut 6
                deal with increment 7
                deal into new stack",
            ),
            &mut cards,
        );
        assert_eq!(cards, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(
            &input_generator(
                "deal with increment 7
                deal with increment 9
                cut -2",
            ),
            &mut cards,
        );
        assert_eq!(cards, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(
            &input_generator(
                "deal into new stack
                cut -2
                deal with increment 7
                cut 8
                cut -4
                deal with increment 7
                cut 3
                deal with increment 9
                deal with increment 3
                cut -1",
            ),
            &mut cards,
        );
        assert_eq!(cards, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn check_part2() {
        assert_eq!(calc_shuffle(&input_generator(""), 10, 0), 0);
        assert_eq!(calc_shuffle(&input_generator(""), 10, 3), 3);
        assert_eq!(calc_shuffle(&input_generator("deal with increment 3"), 10, 0), 0);
        assert_eq!(calc_shuffle(&input_generator("deal with increment 3"), 10, 3), 1);
        assert_eq!(
            calc_shuffle(
                &input_generator(
                    "deal with increment 7
        deal into new stack
        deal into new stack"
                ),
                10,
                0
            ),
            0
        );
        assert_eq!(
            calc_shuffle(
                &input_generator(
                    "deal with increment 7
        deal into new stack
        deal into new stack"
                ),
                10,
                3
            ),
            9
        );
        assert_eq!(
            calc_shuffle(
                &input_generator(
                    "cut 6
                deal with increment 7
                deal into new stack",
                ),
                10,
                0
            ),
            3
        );
        assert_eq!(
            calc_shuffle(
                &input_generator(
                    "cut 6
                    deal with increment 7
                    deal into new stack",
                ),
                10,
                3
            ),
            4
        );

        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(
            &input_generator(
                "deal with increment 7
                deal with increment 9
                cut -2",
            ),
            &mut cards,
        );
        assert_eq!(cards, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let mut cards: Vec<usize> = (0..10).collect();
        shuffle(
            &input_generator(
                "deal into new stack
                cut -2
                deal with increment 7
                cut 8
                cut -4
                deal with increment 7
                cut 3
                deal with increment 9
                deal with increment 3
                cut -1",
            ),
            &mut cards,
        );
        assert_eq!(cards, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
