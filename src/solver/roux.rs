use crate::cube;
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

    use Operation::*;

    info!("FB/1");
    let subgoal = cube![
        * * * ;
        * * * ;
        * * * ;
        * * * * * * * * * * * * ;
        R * * * * * * * * * B B ;
        R * * * * * * * * * B B ;
        W * * ;
        W * * ;
        * * * ;
    ];
    let allowed_ops = vec![
        Front(true),
        Front(false),
        Back(true),
        Back(false),
        Up(true),
        Up(false),
        Down(true),
        Down(false),
        Right(true),
        Right(false),
        Left(true),
        Left(false),
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
    let subgoal = cube![
        * * * ;
        * * * ;
        * * * ;
        * * * * * * * * * * * * ;
        R * * * * * * * O B B B ;
        R * * * * * * * O B B B ;
        W * * ;
        W * * ;
        W * * ;
    ];
    let allowed_ops = vec![
        Back(true),
        Back(false),
        Up(true),
        Up(false),
        Right(true),
        Right(false),
        RightDouble(true),
        RightDouble(false),
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
    let subgoal = cube![
        * * * ;
        * * * ;
        * * * ;
        * * * * * * * * * * * * ;
        R * R G G * * * O B B B ;
        R * R G G * * * O B B B ;
        W * W ;
        W * W ;
        W * * ;
    ];
    let allowed_ops = vec![
        Up(true),
        Up(false),
        Right(true),
        Right(false),
        RightDouble(true),
        RightDouble(false),
        Middle(true),
        Middle(false),
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
    let subgoal = cube![
        * * * ;
        * * * ;
        * * * ;
        * * * * * * * * * * * * ;
        R * R G G G O * O B B B ;
        R * R G G G O * O B B B ;
        W * W ;
        W * W ;
        W * W ;
    ];
    let allowed_ops = vec![
        Up(true),
        Up(false),
        Right(true),
        Right(false),
        RightDouble(true),
        RightDouble(false),
        Middle(true),
        Middle(false),
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

    let subgoal = cube![
        Y Y Y ;
        Y Y Y ;
        Y Y Y ;
        R R R G G G O O O B B B ;
        R R R G G G O O O B B B ;
        R R R G G G O O O B B B ;
        W W W ;
        W W W ;
        W W W ;
    ];
    let allowed_ops = vec![
        Front(true),
        Front(false),
        Up(true),
        Up(false),
        Right(true),
        Right(false),
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
    // let subgoal = cube![
    //     Y * Y ;
    //     Y * Y ;
    //     Y * Y ;
    //     R * R G G G O * O B B B ;
    //     R * R G G G O * O B B B ;
    //     R * R G G G O * O B B B ;
    //     W * W ;
    //     W * W ;
    //     W * W ;
    // ];
    // let allowed_ops = vec![
    //     Up(true),
    //     Up(false),
    //     Middle(true),
    //     Middle(false),
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
    let subgoal = cube![
        Y Y Y ;
        Y Y Y ;
        Y Y Y ;
        R R R G G G O O O B B B ;
        R R R G G G O O O B B B ;
        R R R G G G O O O B B B ;
        W W W ;
        W W W ;
        W W W ;
    ];
    let allowed_ops = vec![Up(true), Up(false), Middle(true), Middle(false)];
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
