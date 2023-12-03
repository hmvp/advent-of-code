use aoc::Input;

aoc::parts!(1, 2);


struct Position {
    column :usize,
    row: usize
}


impl Position {
    fn new(column: usize, row: usize) -> Self {
        Position {column, row}
    }
}
enum EntryType {
    // None,
    Symbol(char),
    Number(u32),
}

struct Entry {
    etype: EntryType,
    positions: Vec<Position>,
}

impl Entry {
    fn unwrap_nr(&self) -> u32 {
        if let EntryType::Number(n) = self.etype {
            n
        } else {
            panic!()
        }
    }

    fn is_close(&self, other: &Entry) -> bool {
        for position in &self.positions {
            for other_position in &other.positions {
                if position.row.abs_diff(other_position.row) > 1{
                    return false
                }

                if position.column.abs_diff(other_position.column) <= 1{
                    return true
                }
            }
        }
        false
    }
}

struct Schematic {
    entries: Vec<Entry>,
}

impl Schematic {
    fn new() -> Self {
        Schematic { entries: Vec::new() }
    }

    fn part_number_sum(self) -> u32 {

        let (mut numbers, symbols) = self.entries.iter().partition::<Vec<_>,_>(|item|matches!(item.etype, EntryType::Number(_)));

        numbers.drain(..).filter(|number|{
            for symbol in &symbols {
                if symbol.is_close(number) {
                    return true
                }
            }
            false
        }).map(|e|if let EntryType::Number(n) = e.etype {n} else {0}).sum()
    }

    fn gear_sum(self)->u32 {
        let (numbers, mut symbols) = self.entries.iter().partition::<Vec<_>,_>(|item|matches!(item.etype, EntryType::Number(_)));

        symbols.drain(..).filter(|symbol|if let EntryType::Symbol(c) = symbol.etype { c == '*'} else {false}).map(|symbol|{
            let mut close_numbers = Vec::new();
            for number in &numbers {
                if symbol.is_close(number) {
                    close_numbers.push(number.unwrap_nr());
                }
            }
            if close_numbers.len() == 2 {
                close_numbers[0] * close_numbers[1]
            } else {
                0
            }
        }).sum()
    }
}

fn parse(input: Input) -> Schematic {
    let mut schematic = Schematic::new();
    for (row, line) in input.lines().enumerate() {
        let mut is_number_in_progress: bool = false;
        for (column, c) in line.chars().enumerate() {
            match (c, is_number_in_progress) {
                ('.', _) => {
                    // schematic.entries.push(Entry {
                    //     etype: EntryType::None,
                    //     positions: vec![Position::new(column, row)],
                    // });
                    is_number_in_progress = false;
                }
                (c, _) if !c.is_numeric() => {
                    schematic.entries.push(Entry {
                        etype: EntryType::Symbol(c),
                        positions: vec![Position::new(column, row)],
                    });
                    is_number_in_progress = false;
                }
                (c, false) if c.is_numeric() => {
                    schematic.entries.push(Entry {
                        etype: EntryType::Number(c.to_digit(10).unwrap()),
                        positions: vec![Position::new(column, row)],
                    });
                    is_number_in_progress = true;
                }
                (c, true) if c.is_numeric() => {
                    let number_in_progress = schematic.entries.last_mut().unwrap();
                    match number_in_progress {
                        Entry {
                            etype: EntryType::Number(n),
                            positions,
                        } => {
                            *n = *n * 10 + c.to_digit(10).unwrap();
                            positions.push(Position::new(column, row));
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    schematic
}

fn part_1(input: aoc::Input) -> impl ToString {
    let schematic = parse(input);

    schematic.part_number_sum()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let schematic = parse(input);

    schematic.gear_sum()}

