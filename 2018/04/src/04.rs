use std::collections::HashMap;
use std::fmt;

use chrono::{NaiveDate, NaiveDateTime, Timelike};

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy)]
enum ObservationType {
    Start(usize),
    Sleep,
    Awake,
}

#[derive(Debug, Clone, Copy)]
struct Observation {
    timestamp: NaiveDateTime,
    observation_type: ObservationType,
}
struct ObservationHour {
    guard_id: usize,
    date: NaiveDate,
    minutes_awake: [bool; 60],
}

impl fmt::Debug for ObservationHour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = write!(
            f,
            "ObservationHour {{ guard_id: {:?}, date: {:?}, minutes: ",
            self.guard_id, self.date
        );
        for m in &self.minutes_awake {
            result = result.and(write!(f, "{}", if *m { "." } else { "#" }));
        }
        result.and(writeln!(f, " }}"))
    }
}

fn parse_observation(line: &str) -> Observation {
    let (_, line) = line.split_at(1);
    let (timestamp, rest) = line.split_at(16);
    let (_, observation) = rest.split_at(2);

    let observation = if observation.starts_with("falls asleep") {
        ObservationType::Sleep
    } else if observation.starts_with("wakes up") {
        ObservationType::Awake
    } else {
        let (_, guard_id_str) = observation.split(' ').nth(1).unwrap().split_at(1);
        ObservationType::Start(guard_id_str.parse().unwrap())
    };

    Observation {
        timestamp: NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap(),
        observation_type: observation,
    }
}

fn parse_input(input: &str) -> Vec<(usize, [usize; 60])> {
    let mut observations = input.lines().map(parse_observation).collect::<Vec<Observation>>();

    observations.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut result = HashMap::new();
    let mut sleep_start = None;
    let mut last_guard_id = 0;

    for observation in observations {
        println!("{}: {:?}", observation.timestamp, observation.observation_type);
        let day: &mut Option<ObservationHour> = result.entry(observation.timestamp.date()).or_default();
        match observation.observation_type {
            ObservationType::Start(guard_id) => {
                last_guard_id = guard_id;
                sleep_start = None;
            }
            ObservationType::Awake => {
                if let Some(sleep_start) = sleep_start {
                    if let Some(ref mut day) = *day {
                        for minute in sleep_start..(observation.timestamp.minute() as usize) {
                            day.minutes_awake[minute] = false;
                        }
                    } else {
                        unreachable!();
                    }
                }
            }
            ObservationType::Sleep => {
                sleep_start = Some(observation.timestamp.minute() as usize);
                if day.is_none() {
                    *day = Some(ObservationHour {
                        guard_id: last_guard_id,
                        date: observation.timestamp.date(),
                        minutes_awake: [true; 60],
                    });
                }
            }
        }
    }

    let observations = result.drain().filter_map(|e| e.1).collect::<Vec<ObservationHour>>();

    println!("{observations:?}");

    let mut guards: HashMap<usize, [usize; 60]> = HashMap::new();
    for observation in observations {
        let minutes = guards.entry(observation.guard_id).or_insert([0; 60]);

        if observation.guard_id == 1459 {
            println!("{:?}{:?}", &minutes[..30], &minutes[30..]);
            println!("{observation:?}");
        }

        for min in 0..60 {
            if !observation.minutes_awake[min] {
                minutes[min] += 1;
            }
        }
    }
    guards.drain().collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let guards = &parse_input(input.raw());

    let mut sleepiest_guard = (0, 0usize, 0);
    for (guard_id, minutes) in guards {
        let (sleepiest_minute, _) = minutes.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        let minutes_asleep = minutes.iter().sum();
        println!(
            "{} {}    {:?} {:?}{:?}",
            sleepiest_guard.1,
            minutes_asleep,
            guard_id,
            &minutes[..30],
            &minutes[30..]
        );
        if sleepiest_guard.1 < minutes_asleep {
            sleepiest_guard = (*guard_id, minutes_asleep, sleepiest_minute);
        }
    }
    println!("{} {} {}", sleepiest_guard.0, sleepiest_guard.1, sleepiest_guard.2);
    sleepiest_guard.0 * sleepiest_guard.2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let guards = &parse_input(input.raw());

    let mut sleepiest_guard = (0, 0usize, 0);
    for (guard_id, minutes) in guards {
        let (sleepiest_minute, minutes_asleep) = minutes.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        println!(
            "{} {}    {:?} {:?}{:?}",
            sleepiest_guard.1,
            minutes_asleep,
            guard_id,
            &minutes[..30],
            &minutes[30..]
        );
        if sleepiest_guard.1 < *minutes_asleep {
            sleepiest_guard = (*guard_id, *minutes_asleep, sleepiest_minute);
        }
    }
    println!("{} {} {}", sleepiest_guard.0, sleepiest_guard.1, sleepiest_guard.2);
    sleepiest_guard.0 * sleepiest_guard.2
}
