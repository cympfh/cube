mod entities;
mod util;
use crate::entities::{Cube, Operation};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    up: bool,
    #[structopt(short, long)]
    down: bool,
    #[structopt(short, long)]
    front: bool,
    #[structopt(short, long)]
    back: bool,
    #[structopt(short, long)]
    left: bool,
    #[structopt(short, long)]
    right: bool,
    #[structopt(short, long)]
    middle: bool,

    #[structopt(short, long)]
    verbose: bool,
}

pub fn solve(
    state: &Cube,
    goal: &Cube,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
) -> Option<Vec<Operation>> {
    if state.matched(goal) {
        return Some(vec![]);
    }
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), state.clone(), vec![]));
    while let Some((_, c, ops)) = q.pop() {
        if c.matched(goal) {
            return Some(ops);
        }
        if ops.len() >= max_depth {
            continue;
        }
        let last: Option<Operation> = if ops.is_empty() {
            None
        } else {
            Some(ops[ops.len() - 1])
        };
        for &op in allowed_ops.iter() {
            if last == Some(op.rev()) {
                continue;
            }
            let mut c = c.clone();
            c.apply(op);
            let mut ops = ops.clone();
            ops.push(op);
            q.push((Reverse(ops.len()), c, ops));
        }
    }
    None
}

pub fn showops(ops: &Vec<Operation>) -> String {
    let mut line = String::new();
    for op in ops {
        line.push_str(format!("{}", op).as_str());
    }
    line
}

fn main() {
    let cube = Cube::read();
    let goal = Cube::read();
    use Operation::*;
    println!("Init:\n{}", &cube);
    println!("Goal:\n{}", &goal);
    if let Some(ops) = solve(
        &cube,
        &goal,
        vec![X(true), X(false), Y(true), Y(false), Z(true), Z(false)],
        3,
    ) {
        println!("Solved: {}", showops(&ops));
    } else {
        println!("Not Solved");
    }
}
