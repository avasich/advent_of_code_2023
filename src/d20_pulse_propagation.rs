use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;
use ModuleKind::*;

use crate::utils::{Day, Task};

#[derive(Copy, Clone, Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct Module<'a> {
    output: HashSet<&'a str>,
    kind: ModuleKind<'a>,
}

impl<'a> Module<'a> {
    fn from_string(s: &'a str) -> (&'a str, Self) {
        let (name, output) = s.split_once(" -> ").unwrap();
        let output = output.split(", ").collect();

        let (name, kind) = match (&name[1..], &name[0..1]) {
            (name, "%") => (name, FlipFlop(Default::default())),
            (name, "&") => (name, Conjunction(Default::default())),
            _ => (name, Broadcaster),
        };

        (name, Self { output, kind })
    }

    fn update(&self, from: &'a str, pulse: Pulse) -> Option<Pulse> {
        match &self.kind {
            FlipFlop(_) if matches!(pulse, Pulse::High) => None,
            FlipFlop(is_on) => {
                let was_on = is_on.replace_with(|on| !*on);
                match !was_on {
                    true => Some(Pulse::High),
                    false => Some(Pulse::Low),
                }
            }
            Endpoint(low_count) => {
                // println!("endpoint hit from {from} with {pulse:?}");
                if !matches!(pulse, Pulse::High) {
                    low_count.replace_with(|count| *count + 1);
                }
                None
            }
            Broadcaster => Some(Pulse::Low),
            Conjunction(inputs) => {
                let inputs = &mut *inputs.borrow_mut();
                inputs.insert(from, pulse);
                match inputs.values().all(|p| matches!(p, Pulse::High)) {
                    true => Some(Pulse::Low),
                    false => Some(Pulse::High),
                }
            }
        }
    }

    fn endpoint() -> Self {
        Self {
            output: Default::default(),
            kind: Endpoint(Default::default()),
        }
    }
}

#[derive(Debug)]
enum ModuleKind<'a> {
    Endpoint(RefCell<u32>), // low pulse hit counter
    Broadcaster,
    FlipFlop(RefCell<bool>),                       // on/off
    Conjunction(RefCell<HashMap<&'a str, Pulse>>), // input history
}

struct System<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> System<'a> {
    fn new(lines: &'a [String]) -> Self {
        let mut modules: HashMap<&'a str, Module<'a>> =
            lines.iter().map(|line| Module::from_string(line)).collect();
        let mut endpoints = vec![];

        for (name, module) in &modules {
            for &connection in &module.output {
                match modules.get(connection) {
                    None => endpoints.push((connection, Module::endpoint())),
                    Some(module) => {
                        if let Conjunction(inputs) = &module.kind {
                            inputs.borrow_mut().insert(name, Pulse::Low);
                        }
                    }
                }
            }
        }

        endpoints.into_iter().for_each(|(name, module)| {
            modules.insert(name, module);
        });

        Self { modules }
    }

    fn push_button(&self) -> (u64, u64) {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back((Pulse::Low, "button", "broadcaster"));

        let mut highs = 0;
        let mut lows = 0;

        while let Some((pulse, source, dist)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::Low => lows += 1,
                Pulse::High => highs += 1,
            }

            if let Some(module) = self.modules.get(dist) {
                if let Some(pulse) = module.update(source, pulse) {
                    module.output.iter().for_each(|out| {
                        pulse_queue.push_back((pulse, dist, out));
                    });
                }
            }
        }

        (lows, highs)
    }

    fn spam_rx(&self) -> Option<u64> {
        let rx = self.modules.get("rx")?;
        if let Endpoint(counter) = &rx.kind {
            while *counter.borrow() == 0 {
                self.push_button();
            }
            let a = (0..)
                .inspect(|_| {
                    self.push_button();
                })
                .find(|_| *counter.borrow() != 0)
                .unwrap();
            return Some(a);
        }

        Some(0)
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.modules
            .iter()
            .for_each(|(k, v)| println!("{k:?}: {v:?}"));
    }
}

fn part_1(filename: &str) -> u64 {
    let lines = crate::utils::read_lines(filename).collect_vec();
    let system = System::new(&lines);
    // system.print();
    let (l, h) = (0..1000)
        .map(|_| system.push_button())
        .fold((0, 0), |acc, lh| (acc.0 + lh.0, acc.1 + lh.1));
    l * h
}

fn part_2(filename: &str) -> u64 {
    let lines = crate::utils::read_lines(filename).collect_vec();
    let system = System::new(&lines);
    system.spam_rx().unwrap()
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            examples: vec![
                "./inputs/day_20/example_01.txt",
                "./inputs/day_20/example_02.txt",
            ],
            task: "./inputs/day_20/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec![],
            task: "./inputs/day_20/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d20_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(32000000, res);

        let res = solution.part_1.run_example(1);
        assert_eq!(11687500, res);
    }

    #[test]
    fn p2_example_test() {
        // let solution = solution();
        //
        // let res = solution.part_2.run_example(0);
        // assert_eq!(11687500, res);
    }
}
