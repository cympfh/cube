mod entities;
mod util;
use crate::entities::{Cube, Operation};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

#[cfg(test)]
mod test {
    use crate::{Cube, Operation};
    use Operation::*;
    #[test]
    fn test_pll() {
        let c = Cube::from(&vec![
            "YYY",
            "YYY",
            "YYY",
            "RRRGGGOOOBBB",
            "RRRGGGOOOBBB",
            "RRRGGGOOOBBB",
            "WWW",
            "WWW",
            "WWW",
        ]);
        {
            // J-perm
            let mut c = c.clone();
            c.apply(Right(true));
            c.apply(Up(true));
            c.apply(Right(false));
            c.apply(Front(false));
            c.apply(Right(true));
            c.apply(Up(true));
            c.apply(Right(false));
            c.apply(Up(false));
            c.apply(Right(false));
            c.apply(Front(true));
            c.apply(Right(true));
            c.apply(Right(true));
            c.apply(Up(false));
            c.apply(Right(false));
            c.apply(Up(false));
            let d = Cube::from(&vec![
                "YYY",
                "YYY",
                "YYY",
                "RGGORRGOOBBB",
                "RRRGGGOOOBBB",
                "RRRGGGOOOBBB",
                "WWW",
                "WWW",
                "WWW",
            ]);
            assert!(c.matched(&d));
        }
        {
            // Z-perm
            let mut c = c.clone();
            c.apply(Middle(false));
            c.apply(Up(false));
            for _ in 0..2 {
                c.apply(Middle(false));
                c.apply(Middle(false));
                c.apply(Up(false));
            }
            c.apply(Middle(false));
            c.apply(Up(false));
            c.apply(Up(false));
            c.apply(Middle(false));
            c.apply(Middle(false));
            c.apply(Up(true));
            let d = Cube::from(&vec![
                "YYY",
                "YYY",
                "YYY",
                "RGRGRGOBOBOB",
                "RRRGGGOOOBBB",
                "RRRGGGOOOBBB",
                "WWW",
                "WWW",
                "WWW",
            ]);
            assert!(c.matched(&d));
        }
    }
    #[test]
    fn test_f2l() {
        let mut c = Cube::from(&vec![
            "***",
            "***",
            "***",
            "************",
            "RRGRGGOOOBBB",
            "RRRGGGOOOBBB",
            "WWW",
            "WWW",
            "WWW",
        ]);
        c.apply(Right(false));
        c.apply(Front(true));
        c.apply(Right(true));
        c.apply(Front(false));
        c.apply(Right(true));
        c.apply(Up(false));
        c.apply(Right(false));
        c.apply(Up(true));
        c.apply(Right(true));
        c.apply(Up(false));
        c.apply(Right(false));
        c.apply(Up(false));
        c.apply(Up(false));
        c.apply(Right(true));
        c.apply(Up(false));
        c.apply(Right(false));
        let d = Cube::from(&vec![
            "***",
            "***",
            "***",
            "************",
            "RRRGGGOOOBBB",
            "RRRGGGOOOBBB",
            "WWW",
            "WWW",
            "WWW",
        ]);
        assert!(c.matched(&d));
    }
}
