use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[allow(unused)]
pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("error reading file");
    io::BufReader::new(file).lines().map(Result::unwrap)
}

pub struct Task<Out: std::fmt::Display> {
    pub example: &'static str,
    pub task: &'static str,
    pub run: fn(&str) -> Out,
}

impl<Out: std::fmt::Display> Task<Out> {
    pub fn run_example(&self) -> Out {
        (self.run)(self.example)
    }

    pub fn run_task(&self) {
        let res = (self.run)(self.task);
        println!("{res}");
    }
}

pub struct Day<Out1, Out2>
where
    Out1: std::fmt::Display,
    Out2: std::fmt::Display,
{
    pub part_1: Task<Out1>,
    pub part_2: Task<Out2>,
}

pub trait Solution {
    fn run_part_1(&self);
    fn run_part_2(&self);
}

impl<Out1, Out2> Solution for Day<Out1, Out2>
where
    Out1: std::fmt::Display,
    Out2: std::fmt::Display,
{
    fn run_part_1(&self) {
        self.part_1.run_task();
    }

    fn run_part_2(&self) {
        self.part_2.run_task();
    }
}
