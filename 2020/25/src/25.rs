aoc::parts!(1);

const MAGIC_NR: usize = 20201227;

fn part_1(input: aoc::Input) -> impl ToString {
    let input: Vec<_> = input.lines().flat_map(str::parse).collect();


    let card_loop_size = find_loop_size(input[0]);
    let door_loop_size = find_loop_size(input[1]);
    dbg!(&input, card_loop_size,door_loop_size);

    transform(input[0], door_loop_size)
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut n = 1;

    for _ in 0..loop_size {
        n *= subject_number;
        n %= MAGIC_NR;
    }
    n
}

fn find_loop_size(public_key: usize) -> usize {
    let mut n = 1;

    for i in 1.. {
        n *= 7;
        n %= MAGIC_NR;

        if public_key == n {
            return i
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_transform() {
        assert_eq!(transform(7, 8), 5764801);
    }
}
