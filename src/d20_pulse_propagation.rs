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

    fn pulse(&self, src_name: &'a str, pulse: Pulse) -> Option<Pulse> {
        match &self.kind {
            FlipFlop(_) if matches!(pulse, Pulse::High) => None,
            FlipFlop(is_on) => match !is_on.replace_with(|on| !*on) {
                true => Some(Pulse::High),
                false => Some(Pulse::Low),
            },
            Conjunction(inputs) => {
                let inputs = &mut *inputs.borrow_mut();
                inputs.insert(src_name, pulse);
                match inputs.values().all(|p| matches!(p, Pulse::High)) {
                    true => Some(Pulse::Low),
                    false => Some(Pulse::High),
                }
            }
            Endpoint(low_count) => {
                if matches!(pulse, Pulse::Low) {
                    *low_count.borrow_mut() += 1;
                }
                None
            }
            Broadcaster => Some(Pulse::Low),
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
    Broadcaster,
    // low pulse hit counter
    Endpoint(RefCell<u32>),
    // on/off
    FlipFlop(RefCell<bool>),
    // input history
    Conjunction(RefCell<HashMap<&'a str, Pulse>>),
}

struct System<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> System<'a> {
    fn new(lines: &'a [String]) -> Self {
        let mut modules: HashMap<&'a str, Module<'a>> = lines
            .iter()
            .map(String::as_str)
            .map(Module::from_string)
            .collect();
        let mut endpoints = vec![];

        for (src_name, src_mod) in &modules {
            for &dist_name in &src_mod.output {
                match modules.get(dist_name).map(|dist_mod| &dist_mod.kind) {
                    None => endpoints.push((dist_name, Module::endpoint())),
                    Some(Conjunction(inputs)) => {
                        inputs.borrow_mut().insert(src_name, Pulse::Low);
                    }
                    _ => {}
                }
            }
        }

        modules.extend(endpoints);

        Self { modules }
    }

    fn push_button(&self, count: u64, watch: &mut HashMap<&'a str, Option<u64>>) -> (u64, u64) {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back((Pulse::Low, "button", "broadcaster"));

        let mut lows = 0;
        let mut highs = 0;

        while let Some((pulse, src_name, dist_name)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::Low => lows += 1,
                Pulse::High => {
                    if let Some(None) = watch.get(src_name) {
                        watch.insert(src_name, Some(count));
                    }
                    highs += 1;
                }
            }

            let dist_mod = self.modules.get(dist_name).unwrap();
            if let Some(pulse) = dist_mod.pulse(src_name, pulse) {
                dist_mod.output.iter().for_each(|out_name| {
                    pulse_queue.push_back((pulse, dist_name, out_name));
                });
            }
        }

        (lows, highs)
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
    let mut map = HashMap::new();
    let (l, h) = (0..1000)
        .map(|_| system.push_button(0, &mut map))
        .fold((0, 0), |acc, count| (acc.0 + count.0, acc.1 + count.1));
    l * h
}

fn part_2(filename: &str) -> u64 {
    let lines = crate::utils::read_lines(filename).collect_vec();
    let system = System::new(&lines);
    let mut watch: HashMap<_, _> = vec!["nx", "sp", "cc", "jq"]
        .into_iter()
        .map(|name| (name, None))
        .collect();

    // yeah, but why?
    for i in 1.. {
        if watch.values().all(Option::is_some) {
            break;
        } else {
            system.push_button(i, &mut watch);
        }
    }

    watch
        .values()
        .flatten()
        .fold(1, |acc, val| num::integer::lcm(acc, *val))
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
