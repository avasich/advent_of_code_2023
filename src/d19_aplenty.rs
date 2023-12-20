use std::collections::HashMap;

use itertools::Itertools;
use Condition::*;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone, Default)]
struct PartRange {
    vals: [Option<(u64, u64)>; 4],
}

impl PartRange {
    fn parse_part(s: String) -> Self {
        let mut vs = s[1..(s.len() - 1)]
            .split(',')
            .flat_map(|val| val[2..].parse::<u64>())
            .map(|v| (v, v));
        let vals = [vs.next(), vs.next(), vs.next(), vs.next()];
        Self { vals }
    }

    fn get_field(&self, f: Field) -> Option<(u64, u64)> {
        self.vals[f as usize]
    }

    fn with_field(&self, f: Field, v: Option<(u64, u64)>) -> Self {
        let mut range = *self;
        range.vals[f as usize] = v;
        range
    }

    fn is_empty(&self) -> bool {
        self.vals.iter().any(Option::is_none)
    }

    fn product(&self) -> u64 {
        self.vals
            .iter()
            .flatten()
            .map(|(v1, v2)| v2 - v1 + 1)
            .product()
    }

    fn sum(&self) -> u64 {
        self.vals.iter().flatten().map(|(v, _)| *v).sum()
    }

    fn partition(self, c: Condition) -> (Self, Self) {
        if self.is_empty() {
            return (Self::default(), Self::default());
        }

        match c {
            LT(f, p) => {
                let (a, b) = self.get_field(f).unwrap();
                let less = (a < p).then_some((a, (p - 1).min(b)));
                let greater_or_equal = (p <= b).then_some((p.max(a), b));
                (
                    self.with_field(f, less),
                    self.with_field(f, greater_or_equal),
                )
            }
            GT(f, p) => {
                let (a, b) = self.get_field(f).unwrap();
                let less_or_equal = (a <= p).then_some((a, p.min(b)));
                let greater = (p < b).then_some(((p + 1).max(a), b));
                (
                    self.with_field(f, greater),
                    self.with_field(f, less_or_equal),
                )
            }
            True => (self, Self::default()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Field {
    X = 0,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
enum Condition {
    LT(Field, u64),
    GT(Field, u64),
    True,
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    target: String,
}

impl Rule {
    fn from_string(s: &str) -> Self {
        match s.split_once(':') {
            None => Self {
                condition: True,
                target: s.to_string(),
            },
            Some((condition, target)) => {
                let field = match &condition[0..1] {
                    "x" => Field::X,
                    "m" => Field::M,
                    "a" => Field::A,
                    "s" => Field::S,
                    _ => unreachable!(),
                };
                let val = condition[2..].parse().unwrap();
                let condition = match &condition[1..2] {
                    ">" => GT(field, val),
                    "<" => LT(field, val),
                    _ => unreachable!(),
                };

                Self {
                    condition,
                    target: target.to_string(),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_string(s: String) -> Self {
        let (name, rules) = s.split_once('{').unwrap();
        let rules = rules[..rules.len() - 1]
            .split(',')
            .map(Rule::from_string)
            .collect_vec();
        Self {
            name: name.to_string(),
            rules,
        }
    }
}

struct Validator {
    workflows: HashMap<String, Workflow>,
}

impl Validator {
    fn new(workflows: Vec<Workflow>) -> Self {
        let workflows = workflows
            .into_iter()
            .map(|wf| (wf.name.clone(), wf))
            .collect();
        Self { workflows }
    }

    fn count_matching(&self, wf_name: &str, rule_n: usize, range: PartRange) -> u64 {
        if wf_name == "R" || range.is_empty() {
            return 0;
        } else if wf_name == "A" {
            return range.product();
        }

        let wf = self.workflows.get(wf_name).unwrap();
        let Rule { target, condition } = &wf.rules[rule_n];

        let (matching, rest) = range.partition(*condition);
        let matching = self.count_matching(target, 0, matching);
        let rest = self.count_matching(wf_name, rule_n + 1, rest);

        matching + rest
    }
}

fn part_1(filename: &str) -> u64 {
    let mut lines = crate::utils::read_lines(filename);
    let mut workflows = vec![];
    loop {
        match lines.next() {
            Some(line) if line.is_empty() => break,
            Some(line) => workflows.push(Workflow::from_string(line)),
            _ => unreachable!(),
        }
    }
    let validator = Validator::new(workflows);

    lines
        .map(PartRange::parse_part)
        .filter(|part| validator.count_matching("in", 0, *part) != 0)
        .map(|part| part.sum())
        .sum()
}

fn part_2(filename: &str) -> u64 {
    let workflows = crate::utils::read_lines(filename)
        .take_while(|line| !line.is_empty())
        .map(Workflow::from_string)
        .collect_vec();

    Validator::new(workflows).count_matching("in", 0, PartRange {
        vals: [Some((1, 4000)); 4],
    })
}

pub fn solution() -> Day<u64, u64> {
    Day {
        part_1: Task {
            examples: vec!["./inputs/day_19/example_01.txt"],
            task: "./inputs/day_19/task.txt",
            run: part_1,
        },
        part_2: Task {
            examples: vec!["./inputs/day_19/example_01.txt"],
            task: "./inputs/day_19/task.txt",
            run: part_2,
        },
    }
}

#[cfg(test)]
mod d19_tests {
    use super::*;

    #[test]
    fn p1_example_test() {
        let solution = solution();
        let res = solution.part_1.run_example(0);
        assert_eq!(19114, res);
    }

    #[test]
    fn p2_example_test() {
        let solution = solution();

        let res = solution.part_2.run_example(0);
        assert_eq!(167409079868000, res);
    }
}
