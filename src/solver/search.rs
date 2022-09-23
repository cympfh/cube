use crate::entities::*;
use crate::trace;
use log::info;
use std::cmp::Reverse;
use std::collections::*;
use Operation::*;

pub fn search(
    init_state: &Cube,
    goal: &Cube,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    num: usize,
    verbose: bool,
) -> Vec<Ops> {
    let xyz_map = xyz(goal);
    let exact = !init_state.has_wildcard() && !goal.has_wildcard();
    if verbose {
        trace!(exact);
    }
    solve(
        init_state,
        &xyz_map,
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
fn solve(
    init_state: &Cube,
    xyz_map: &BTreeMap<Cube, Ops>,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    num: usize,
    exact: bool,
    verbose: bool,
) -> Vec<Ops> {
    let mut cubes_from_start = BTreeMap::new();
    let mut cubes_from_goal = BTreeMap::new();
    const MAX_MAP_SIZE: usize = 20_000;

    let mut q = BinaryHeap::new();
    q.push((Reverse((0, true)), init_state.clone(), Ops::default(), true));
    for (c, ops) in xyz_map.iter() {
        q.push((
            Reverse((ops.weight(), false)),
            c.clone(),
            ops.clone(),
            false,
        ));
    }

    let mut solutions = vec![];
    let mut solutionset = BTreeSet::new();
    let add_solution =
        |solutions: &mut Vec<Ops>, solutionset: &mut BTreeSet<Ops>, solution: Ops| {
            if solutionset.contains(&solution) {
                return;
            }
            info!("Solution: {}", solution);
            if verbose {
                let c = solution.apply(&init_state);
                info!("Validation:\n{}", c);
            }
            solutions.push(solution.clone());
            solutionset.insert(solution);
        };

    let found = |cube: &Cube, ops: &Ops, cubes_from_goal: &BTreeMap<Cube, Ops>| -> Option<Ops> {
        if exact {
            if let Some(ops_from_goal) = cubes_from_goal.get(&cube) {
                let mut ops = ops.clone();
                ops.extend(&ops_from_goal.rev());
                return Some(ops.clone());
            }
        } else {
            for (d, ops_from_goal) in cubes_from_goal.iter() {
                if cube.matched(d) {
                    let mut ops = ops.clone();
                    ops.extend(&ops_from_goal.rev());
                    return Some(ops.clone());
                }
            }
        }
        None
    };

    let found_reverse =
        |cube: &Cube, ops_from_goal: &Ops, cubes_from_start: &BTreeMap<Cube, Ops>| -> Option<Ops> {
            if exact {
                if let Some(ops) = cubes_from_start.get(&cube) {
                    let mut ops = ops.clone();
                    ops.extend(&ops_from_goal.rev());
                    return Some(ops.clone());
                }
            } else {
                for (d, ops) in cubes_from_start.iter() {
                    if cube.matched(d) {
                        let mut ops = ops.clone();
                        ops.extend(&ops_from_goal.rev());
                        return Some(ops.clone());
                    }
                }
            }
            None
        };

    let mut searching_depth: usize = 0;
    while let Some((_, c, ops, from_start)) = q.pop() {
        if solutions.len() >= num {
            break;
        }
        if verbose && ops.len() > searching_depth {
            searching_depth = ops.len();
            info!("Searching depth: {}", searching_depth);
        }
        if from_start {
            if !exact && cubes_from_goal.len() > MAX_MAP_SIZE {
                continue;
            }
            if cubes_from_start.contains_key(&c) {
                continue;
            }
            cubes_from_start.insert(c.clone(), ops.clone());
            if let Some(solution) = found(&c, &ops, &cubes_from_goal) {
                add_solution(&mut solutions, &mut solutionset, solution.shorten());
                continue;
            }
        } else {
            if !exact && cubes_from_goal.len() > MAX_MAP_SIZE {
                continue;
            }
            if cubes_from_goal.contains_key(&c) {
                continue;
            }
            cubes_from_goal.insert(c.clone(), ops.clone());
            if let Some(solution) = found_reverse(&c, &ops, &cubes_from_start) {
                add_solution(&mut solutions, &mut solutionset, solution.shorten());
                continue;
            }
        }
        if ops.len() >= max_depth {
            continue;
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
            // Jb is too heavy
            if let Jb(_) = op {
                if !from_start {
                    continue;
                }
                let mut skip = false;
                for &op in ops.data.iter() {
                    if let Jb(_) = op {
                        skip = true;
                    }
                }
                if skip {
                    continue;
                }
            }
            let mut c = c.clone();
            c.apply(op);
            let mut ops = ops.clone();
            ops.push(op);
            q.push((Reverse((ops.weight(), from_start)), c, ops, from_start));
        }
    }
    solutions
}
