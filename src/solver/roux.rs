use crate::entities::*;
use crate::solver::search;
use log::info;

fn search_one(
    init_state: &Cube,
    goal: &Cube,
    allowed_ops: Vec<Operation>,
    max_depth: usize,
    verbose: bool,
) -> Option<Ops> {
    let algs = search(init_state, goal, allowed_ops, max_depth, 1, verbose);
    algs.get(0).cloned()
}

pub fn roux(cube: &Cube, verbose: bool) -> Option<Ops> {
    let mut cube: Cube = cube.clone();
    let mut algorithm = Ops::default();

    info!("FB/1");
    let subgoal = Cube::from(&vec![
        "***",
        "***",
        "***",
        "************",
        "R*********BB",
        "R*********BB",
        "W**",
        "W**",
        "***",
    ]);
    let allowed_ops = vec![
        Operation::Front(true),
        Operation::Front(false),
        Operation::Back(true),
        Operation::Back(false),
        Operation::Up(true),
        Operation::Up(false),
        Operation::Down(true),
        Operation::Down(false),
        Operation::Right(true),
        Operation::Right(false),
        Operation::Left(true),
        Operation::Left(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 8, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("FB/2");
    let subgoal = Cube::from(&vec![
        "***",
        "***",
        "***",
        "************",
        "R*******OBBB",
        "R*******OBBB",
        "W**",
        "W**",
        "W**",
    ]);
    let allowed_ops = vec![
        Operation::Back(true),
        Operation::Back(false),
        Operation::Up(true),
        Operation::Up(false),
        Operation::Right(true),
        Operation::Right(false),
        Operation::RightDouble(true),
        Operation::RightDouble(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 8, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("SB/1");
    let subgoal = Cube::from(&vec![
        "***",
        "***",
        "***",
        "************",
        "R*RGG***OBBB",
        "R*RGG***OBBB",
        "W*W",
        "W*W",
        "W**",
    ]);
    let allowed_ops = vec![
        Operation::Up(true),
        Operation::Up(false),
        Operation::Right(true),
        Operation::Right(false),
        Operation::RightDouble(true),
        Operation::RightDouble(false),
        Operation::Middle(true),
        Operation::Middle(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 8, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("SB/2");
    let subgoal = Cube::from(&vec![
        "***",
        "***",
        "***",
        "************",
        "R*RGGGO*OBBB",
        "R*RGGGO*OBBB",
        "W*W",
        "W*W",
        "W*W",
    ]);
    let allowed_ops = vec![
        Operation::Up(true),
        Operation::Up(false),
        Operation::Right(true),
        Operation::Right(false),
        Operation::RightDouble(true),
        Operation::RightDouble(false),
        Operation::Middle(true),
        Operation::Middle(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 8, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("CMLL");
    let mut cm = cube.clone();
    cm.up[(0, 1)] = Color::Wildcard;
    cm.up[(1, 0)] = Color::Wildcard;
    cm.up[(1, 1)] = Color::Wildcard;
    cm.up[(1, 2)] = Color::Wildcard;
    cm.up[(2, 1)] = Color::Wildcard;
    cm.front[(0, 1)] = Color::Wildcard;
    cm.front[(1, 1)] = Color::Wildcard;
    cm.front[(2, 1)] = Color::Wildcard;
    cm.back[(0, 1)] = Color::Wildcard;
    cm.back[(1, 1)] = Color::Wildcard;
    cm.back[(2, 1)] = Color::Wildcard;
    cm.down[(0, 1)] = Color::Wildcard;
    cm.down[(1, 1)] = Color::Wildcard;
    cm.down[(2, 1)] = Color::Wildcard;
    cm.right[(0, 1)] = Color::Wildcard;
    cm.left[(0, 1)] = Color::Wildcard;

    let subgoal = Cube::from(&vec![
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
    let allowed_ops = vec![
        Operation::Front(true),
        Operation::Front(false),
        Operation::Up(true),
        Operation::Up(false),
        Operation::Right(true),
        Operation::Right(false),
    ];
    match search_one(&cm, &subgoal, allowed_ops, 10, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    // info!("LSE/UL&UR");
    // let subgoal = Cube::from(&vec![
    //     "Y*Y",
    //     "Y*Y",
    //     "Y*Y",
    //     "R*RGGGO*OBBB",
    //     "R*RGGGO*OBBB",
    //     "R*RGGGO*OBBB",
    //     "W*W",
    //     "W*W",
    //     "W*W",
    // ]);
    // let allowed_ops = vec![
    //     Operation::Up(true),
    //     Operation::Up(false),
    //     Operation::Middle(true),
    //     Operation::Middle(false),
    // ];
    // match search_one(&cube, &subgoal, allowed_ops, 12, verbose) {
    //     Some(alg) => {
    //         algorithm.extend(&alg);
    //         cube = alg.apply(&cube);
    //     }
    //     None => {
    //         return None;
    //     }
    // }

    info!("LSE");
    let subgoal = Cube::from(&vec![
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
    let allowed_ops = vec![
        Operation::Up(true),
        Operation::Up(false),
        Operation::Middle(true),
        Operation::Middle(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 12, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            // cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    Some(algorithm)
}
