#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;
use Condition::*;

use crate::utils::{Day, Task};

#[derive(Debug, Copy, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get(&self, field: &Field) -> u64 {
        match field {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }

    fn satisfies(&self, condition: &Condition) -> bool {
        match condition {
            LT(field, val) => self.get(field) < *val,
            GT(field, val) => self.get(field) > *val,
            True => true,
        }
    }

    fn from_string(s: String) -> Self {
        let mut vs = s[1..(s.len() - 1)]
            .split(',')
            .flat_map(|val| val[2..].parse());
        let x = vs.next().unwrap();
        let m = vs.next().unwrap();
        let a = vs.next().unwrap();
        let s = vs.next().unwrap();

        Self { x, m, a, s }
    }

    fn sum_fields(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
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
    fn send_part(&self, part: &Part) -> &str {
        self.rules
            .iter()
            .find_map(|rule| part.satisfies(&rule.condition).then_some(&rule.target))
            .unwrap()
    }

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

    fn check(&self, part: &Part) -> bool {
        let mut wf_name = "in";
        loop {
            match self.workflows.get(wf_name).unwrap().send_part(part) {
                "A" => return true,
                "R" => return false,
                name => wf_name = name,
            }
        }
    }

    fn foo(
        &self,
        wf: &str,
        (x1, x2): (u64, u64),
        (m1, m2): (u64, u64),
        (a1, a2): (u64, u64),
        (s1, s2): (u64, u64),
    ) -> u64 {
        let wf = self.workflows.get(wf).unwrap();

        for r in &wf.rules {
            match &r.condition {
                LT(f, v) => match f {
                    Field::X => {}
                    Field::M => {}
                    Field::A => {}
                    Field::S => {}
                },
                GT(f, v) => match f {
                    Field::X => {}
                    Field::M => {}
                    Field::A => {}
                    Field::S => {}
                },
                True => {}
            }
        }

        1
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
        .map(Part::from_string)
        .filter(|part| validator.check(part))
        .map(|part| part.sum_fields())
        .sum()
}

fn part_2(filename: &str) -> u64 {
    let workflows = crate::utils::read_lines(filename)
        .map(Workflow::from_string)
        .collect_vec();
    let validator = Validator::new(workflows);
    0
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
