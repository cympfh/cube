mod entities;
mod util;
use crate::entities::{Cube, Operation, Ops};
use std::cmp::Reverse;
use std::collections::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, default_value = "9")]
    max_depth: usize,
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(long, help = "exact equality check")]
    exact: bool,

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
}

fn solve(
    state: &Cube,
    goal: &Cube,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    exact: bool,
    verbose: bool,
) -> Option<Ops> {
    let mut goal_map = xyz(goal);
    solve_wo_xyz(state, &mut goal_map, allowed_ops, max_depth, exact, verbose)
}

/// Set of all cube states from the given state only using xyz
fn xyz(state: &Cube) -> BTreeMap<Cube, Ops> {
    const MAX_DEPTH: usize = 3;
    const ALLOWED_OPS: [Operation; 6] = [
        Operation::X(true),
        Operation::X(false),
        Operation::Y(true),
        Operation::Y(false),
        Operation::Z(true),
        Operation::Z(false),
    ];
    let mut map = BTreeMap::new();
    let mut q = VecDeque::new();
    q.push_back((state.clone(), Ops::default()));
    while let Some((c, ops)) = q.pop_front() {
        if ops.len() < MAX_DEPTH {
            let last = ops.last();
            let last_repeat = ops.last_repeat();
            for &op in ALLOWED_OPS.iter() {
                if last == Some(op.rev()) {
                    continue;
                }
                if last_repeat == Some(op) {
                    continue;
                }
                let mut c = c.clone();
                c.apply(op);
                let mut ops = ops.clone();
                ops.push(op);
                q.push_back((c, ops));
            }
        }
        if !map.contains_key(&c) {
            map.insert(c, ops);
        }
    }
    map
}

/// Bi-direction search with xyz_map
fn solve_wo_xyz(
    state: &Cube,
    goal_map: &mut BTreeMap<Cube, Ops>,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    exact: bool,
    verbose: bool,
) -> Option<Ops> {
    const MAX_GOALMAP_SIZE: usize = 1000;
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), state.clone(), Ops::default(), true));
    for (c, ops) in goal_map.iter() {
        q.push((Reverse(ops.len()), c.clone(), ops.clone(), false));
    }
    let mut searching_depth: usize = 0;
    while let Some((_, c, mut ops, from_start)) = q.pop() {
        if from_start {
            if exact {
                if goal_map.contains_key(&c) {
                    let ops_from_goal = goal_map[&c].clone();
                    ops.extend(&ops_from_goal.rev());
                    return Some(ops);
                }
            } else {
                for (d, ops_from_goal) in goal_map.iter() {
                    if c.matched(d) {
                        ops.extend(&ops_from_goal.rev());
                        return Some(ops);
                    }
                }
            }
        } else {
            if !exact && goal_map.len() > MAX_GOALMAP_SIZE {
                continue;
            }
            goal_map.insert(c.clone(), ops.clone());
        }
        if ops.len() >= max_depth {
            continue;
        }
        let last = ops.last();
        let last_repeat = ops.last_repeat();
        for &op in allowed_ops.iter() {
            if last == Some(op.rev()) {
                continue;
            }
            if last_repeat == Some(op) {
                continue;
            }
            let mut c = c.clone();
            c.apply(op);
            let mut ops = ops.clone();
            ops.push(op);
            q.push((Reverse(ops.len()), c, ops, from_start));
        }
        if verbose && ops.len() > searching_depth {
            searching_depth = ops.len();
            eprintln!("Searching depth: {}", searching_depth);
        }
    }
    None
}

fn main() {
    use Operation::*;

    let opt = Opt::from_args();
    let mut allowed_ops = vec![];
    if opt.up {
        allowed_ops.push(Up(true));
        allowed_ops.push(Up(false));
    }
    if opt.down {
        allowed_ops.push(Down(true));
        allowed_ops.push(Down(false));
    }
    if opt.front {
        allowed_ops.push(Front(true));
        allowed_ops.push(Front(false));
    }
    if opt.back {
        allowed_ops.push(Back(true));
        allowed_ops.push(Back(false));
    }
    if opt.left {
        allowed_ops.push(Left(true));
        allowed_ops.push(Left(false));
    }
    if opt.right {
        allowed_ops.push(Right(true));
        allowed_ops.push(Right(false));
    }
    if opt.middle {
        allowed_ops.push(Middle(true));
        allowed_ops.push(Middle(false));
    }
    if allowed_ops.is_empty() {
        eprintln!("ERROR: No Operations specified");
        return;
    }

    let cube = Cube::read();
    let goal = Cube::read();
    println!("Init:\n{}", &cube);
    println!("Goal:\n{}", &goal);

    if let Some(ops) = solve(
        &cube,
        &goal,
        allowed_ops,
        opt.max_depth,
        opt.exact,
        opt.verbose,
    ) {
        println!("Solved: {}", ops);
        if opt.verbose {
            let c = ops.apply(&cube);
            eprintln!("Validation:\n{}", c);
        }
    } else {
        println!("Not Solved");
    }
}
