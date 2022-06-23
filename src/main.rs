mod entities;
mod util;

use crate::entities::{Cube, Operation, Ops};
use log::{error, info, warn};
use serde_json::json;
use std::cmp::Reverse;
use std::collections::*;
use std::env;
use structopt::StructOpt;
use Operation::*;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, default_value = "9")]
    max_depth: usize,
    #[structopt(short, long, default_value = "1", help = "max num of algorithm")]
    num: usize,
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(short, long)]
    quiet: bool,

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
    init_state: &Cube,
    goal: &Cube,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    num: usize,
    verbose: bool,
) -> Vec<Ops> {
    let mut goal_map = xyz(goal);
    let exact = !init_state.has_wildcard() && !goal.has_wildcard();
    if verbose {
        trace!(exact);
    }
    solve_wo_xyz(
        init_state,
        &mut goal_map,
        allowed_ops,
        max_depth,
        num,
        exact,
        verbose,
    )
}

/// Set of all cube states from the given state only using xyz
fn xyz(state: &Cube) -> BTreeMap<Cube, Ops> {
    const MAX_DEPTH: usize = 3;
    const ALLOWED_OPS: [Operation; 6] = [X(true), X(false), Y(true), Y(false), Z(true), Z(false)];
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
    init_state: &Cube,
    goal_map: &mut BTreeMap<Cube, Ops>,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    num: usize,
    exact: bool,
    verbose: bool,
) -> Vec<Ops> {
    const MAX_GOALMAP_SIZE: usize = 1000;
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), init_state.clone(), Ops::default(), true));
    for (c, ops) in goal_map.iter() {
        q.push((Reverse(ops.len()), c.clone(), ops.clone(), false));
    }

    let mut solutions = vec![];
    let found = |cube: &Cube, ops: &mut Ops, goal_map: &mut BTreeMap<Cube, Ops>| -> Option<Ops> {
        if exact {
            if let Some(ops_from_goal) = goal_map.get(&cube) {
                ops.extend(&ops_from_goal.rev());
                return Some(ops.clone());
            }
        } else {
            for (d, ops_from_goal) in goal_map.iter() {
                if cube.matched(d) {
                    ops.extend(&ops_from_goal.rev());
                    return Some(ops.clone());
                }
            }
        }
        None
    };

    let mut searching_depth: usize = 0;
    while let Some((_, c, mut ops, from_start)) = q.pop() {
        if solutions.len() >= num {
            break;
        }
        if ops.len() > max_depth {
            continue;
        }
        if verbose && ops.len() > searching_depth {
            searching_depth = ops.len();
            info!("Searching depth: {}", searching_depth);
        }
        if from_start {
            if let Some(solution) = found(&c, &mut ops, goal_map) {
                info!("Solution: {}", solution);
                if verbose {
                    let c = solution.apply(&init_state);
                    info!("Validation:\n{}", c);
                }
                solutions.push(solution);
                continue;
            }
        } else {
            if !exact && goal_map.len() > MAX_GOALMAP_SIZE {
                continue;
            }
            goal_map.insert(c.clone(), ops.clone());
        }
        let last = ops.last();
        let last_repeat = ops.last_repeat();
        for &op in allowed_ops.iter() {
            // Dont Canceling Move (e.g. UU')
            if last == Some(op.rev()) {
                continue;
            }
            // Dont repeat Reverse Move (e.g. U'U' is same to UU)
            if last == Some(op) && op.is_reversed() == from_start {
                continue;
            }
            // Dont repeat 3 times (e.g. UUU is same to U')
            if last_repeat == Some(op) {
                continue;
            }
            let mut c = c.clone();
            c.apply(op);
            let mut ops = ops.clone();
            ops.push(op);
            q.push((Reverse(ops.len()), c, ops, from_start));
        }
    }
    solutions
}

fn main() {
    let opt = Opt::from_args();

    env::set_var("RUST_LOG", if !opt.quiet { "info" } else { "error" });
    env_logger::builder()
        .format_target(false)
        .format_indent(Some(0))
        .init();

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
        error!("No Operations specified");
        return;
    }

    if opt.max_depth >= 10 {
        warn!("Too large max_depth: {}", opt.max_depth);
    }

    let cube = Cube::read();
    let goal = Cube::read();
    info!("Init:\n{}", &cube);
    info!("Goal:\n{}", &goal);

    let algorithms = solve(
        &cube,
        &goal,
        allowed_ops,
        opt.max_depth,
        opt.num,
        opt.verbose,
    );
    if !algorithms.is_empty() {
        let mut solutions = vec![];
        for ops in algorithms.iter() {
            solutions.push(json!({
                        "algorithm": format!("{}", ops),
                        "length": ops.len(),
            }));
        }
        println!("{}", json!({ "ok": true, "solutions": solutions, }));
    } else {
        info!("No Solution");
        println!("{}", json!({ "ok": false, "solutions": [], }));
    }
}

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        info!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),* $(,)?) => { trace!(($($xs),*)) }
}
