use crate::cube;
use crate::entities::*;
use crate::solver::search;
use crate::util;
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

pub fn cfop(cube: &Cube, verbose: bool) -> Option<Ops> {
    let mut cube = cube.clone();
    let mut algorithm = Ops::default();

    use Operation::*;

    info!("Cross");
    let mut subcube = cube.clone();
    for ((face0, i0, j0), (face1, i1, j1), (face2, i2, j2)) in util::corners() {
        subcube[face0][(i0, j0)] = Color::Other;
        subcube[face1][(i1, j1)] = Color::Other;
        subcube[face2][(i2, j2)] = Color::Other;
    }
    for ((face0, i0, j0), (face1, i1, j1)) in util::edges() {
        if subcube[face0][(i0, j0)] == Color::White || subcube[face1][(i1, j1)] == Color::White {
            continue;
        }
        subcube[face0][(i0, j0)] = Color::Other;
        subcube[face1][(i1, j1)] = Color::Other;
    }
    let subgoal = cube![
        . . . ;
        . Y . ;
        . . . ;
        . . . . . . . . . . . . ;
        . R . . G . . O . . B . ;
        . R . . G . . O . . B . ;
        . W . ;
        W W W ;
        . W . ;
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
    match search_one(&subcube, &subgoal, allowed_ops, 5, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("F2L/#1");
    let subgoal = cube![
        * * * ;
        * Y * ;
        * * * ;
        * * * * * * * * * * * * ;
        R R * * G * * O * * B B ;
        R R * * G * * O * * B B ;
        W W * ;
        W W W ;
        * W * ;
    ];
    let allowed_ops = vec![
        Front(true),
        Front(false),
        Back(true),
        Back(false),
        Up(true),
        Up(false),
        Right(true),
        Right(false),
        Left(true),
        Left(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 6, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("F2L/#2");
    let subgoal = cube![
        * * * ;
        * Y * ;
        * * * ;
        * * * * * * * * * * * * ;
        R R * * G * * O O B B B ;
        R R * * G * * O O B B B ;
        W W * ;
        W W W ;
        W W * ;
    ];
    let allowed_ops = vec![
        // Front(true),
        // Front(false),
        Back(true),
        Back(false),
        Up(true),
        Up(false),
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

    info!("F2L/#3");
    let subgoal = cube![
        * * * ;
        * Y * ;
        * * * ;
        * * * * * * * * * * * * ;
        R R * * G G O O O B B B ;
        R R * * G G O O O B B B ;
        W W * ;
        W W W ;
        W W W ;
    ];
    let allowed_ops = vec![
        // Front(true),
        // Front(false),
        Back(true),
        Back(false),
        Up(true),
        Up(false),
        Right(true),
        Right(false),
        // Left(true),
        // Left(false),
    ];
    match search_one(&cube, &subgoal, allowed_ops, 6, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("F2L/#4");
    let subgoal = cube![
        * * * ;
        * Y * ;
        * * * ;
        * * * * * * * * * * * * ;
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
    match search_one(&cube, &subgoal, allowed_ops, 6, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("OLL");
    let mut subcube = cube.clone();
    for i in 0..3 {
        for j in 0..3 {
            if subcube.up[(i, j)] != Color::Yellow {
                subcube.up[(i, j)] = Color::Other;
            }
        }
        if subcube.front[(0, i)] != Color::Yellow {
            subcube.front[(0, i)] = Color::Other;
        };
        if subcube.right[(0, i)] != Color::Yellow {
            subcube.right[(0, i)] = Color::Other;
        };
        if subcube.back[(0, i)] != Color::Yellow {
            subcube.back[(0, i)] = Color::Other;
        };
        if subcube.left[(0, i)] != Color::Yellow {
            subcube.left[(0, i)] = Color::Other;
        };
    }
    let subgoal = cube![
        Y Y Y ;
        Y Y Y ;
        Y Y Y ;
        . . . . . . . . . . . . ;
        R R R G G G O O O B B B ;
        R R R G G G O O O B B B ;
        W W W ;
        W W W ;
        W W W ;
    ];
    info!("OLL/UR");
    let allowed_ops = vec![Right(true), Right(false), Up(true), Up(false)];
    match search_one(&subcube, &subgoal, allowed_ops, 8, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            info!("OLL/FUR");
            let allowed_ops = vec![
                Right(true),
                Right(false),
                Up(true),
                Up(false),
                Front(true),
                Front(false),
            ];
            match search_one(&subcube, &subgoal, allowed_ops, 8, verbose) {
                Some(alg) => {
                    algorithm.extend(&alg);
                    cube = alg.apply(&cube);
                }
                None => {
                    info!("OLL/Full");
                    let allowed_ops = vec![
                        Front(true),
                        Front(false),
                        Up(true),
                        Up(false),
                        Right(true),
                        Right(false),
                        RightDouble(true),
                        RightDouble(false),
                        // Left(true),
                        // Left(false),
                    ];
                    match search_one(&subcube, &subgoal, allowed_ops, 8, verbose) {
                        Some(alg) => {
                            algorithm.extend(&alg);
                            cube = alg.apply(&cube);
                        }
                        None => {
                            return None;
                        }
                    }
                }
            }
        }
    }

    info!("PLL");
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
    info!("PLL/MU");
    let allowed_ops = vec![Up(true), Up(false), Middle(true), Middle(false)];
    match search_one(&cube, &subgoal, allowed_ops, 10, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            info!("PLL/FUR");
            let allowed_ops = vec![
                Front(true),
                Front(false),
                Up(true),
                Up(false),
                Right(true),
                Right(false),
            ];
            match search_one(&cube, &subgoal, allowed_ops, 9, verbose) {
                Some(alg) => {
                    algorithm.extend(&alg);
                    cube = alg.apply(&cube);
                }
                None => {
                    info!("PLL/UDR");
                    let allowed_ops = vec![
                        Up(true),
                        Up(false),
                        Down(true),
                        Down(false),
                        Right(true),
                        Right(false),
                    ];
                    match search_one(&cube, &subgoal, allowed_ops, 9, verbose) {
                        Some(alg) => {
                            algorithm.extend(&alg);
                            cube = alg.apply(&cube);
                        }
                        None => {
                            info!("PLL/Full");
                            let allowed_ops = vec![
                                Front(true),
                                Front(false),
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
                        }
                    }
                }
            }
        }
    }

    info!("{}", cube);
    Some(algorithm.shorten())
}
