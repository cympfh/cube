use crate::cube;
use crate::entities::*;
use crate::read;
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

pub fn cfop(cube: &Cube, verbose: bool) -> Option<Ops> {
    let mut cube = cube.clone();
    let mut algorithm = Ops::default();

    use Operation::*;

    info!("Cross");
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
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
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
        . . . ;
        . Y . ;
        . . . ;
        . . . . . . . . . . . . ;
        R R . . G . . O . . B B ;
        R R . . G . . O . . B B ;
        W W . ;
        W W W ;
        . W . ;
    ];
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
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
    match search_one(&subcube, &subgoal, allowed_ops, 6, verbose) {
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
        . . . ;
        . Y . ;
        . . . ;
        . . . . . . . . . . . . ;
        R R . . G . . O O B B B ;
        R R . . G . . O O B B B ;
        W W . ;
        W W W ;
        W W . ;
    ];
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
    let allowed_ops = vec![
        Back(true),
        Back(false),
        Up(true),
        Up(false),
        Right(true),
        Right(false),
        Left(true),
        Left(false),
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

    info!("F2L/#3+#4");
    let subgoal = cube![
        . . . ;
        . Y . ;
        . . . ;
        . . . . . . . . . . . . ;
        R R R G G G O O O B B B ;
        R R R G G G O O O B B B ;
        W W W ;
        W W W ;
        W W W ;
    ];
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
    let allowed_ops = vec![
        Front(true),
        Front(false),
        Back(true),
        Back(false),
        Up(true),
        Up(false),
        Right(true),
        Right(false),
    ];
    match search_one(&subcube, &subgoal, allowed_ops, 6, verbose) {
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
    let sexy = Operation::Compound(
        "Sx".to_string(),
        true,
        read::parse_ops(&"RUR'U'").unwrap().1.data,
    );
    let sledgehammer = Operation::Compound(
        "Sh".to_string(),
        true,
        read::parse_ops(&"R'FRF'").unwrap().1.data,
    );
    let jb = Operation::Compound(
        "Jb".to_string(),
        true,
        read::parse_ops(&"RUR'F'RUR'U'R'FR2U'R'").unwrap().1.data,
    );
    match search_one(&cube, &subgoal, allowed_ops, 7, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            info!("PLL/RUF+Sx+Sh+Jb");
            let allowed_ops = vec![
                Up(true),
                Up(false),
                Front(true),
                Front(false),
                Right(true),
                Right(false),
                sexy.clone(),
                sexy.rev(),
                sledgehammer.clone(),
                sledgehammer.rev(),
                jb,
            ];
            match search_one(&cube, &subgoal, allowed_ops, 5, verbose) {
                Some(alg) => {
                    algorithm.extend(&alg);
                    cube = alg.apply(&cube);
                }
                None => {
                    // This can solve all PLL speedy!!
                    // But this may be unreachable...
                    info!("PLL/U+Sx+Sh");
                    let allowed_ops = vec![
                        Up(true),
                        Up(false),
                        sexy.clone(),
                        sexy.rev(),
                        sledgehammer.clone(),
                        sledgehammer.rev(),
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
                }
            }
        }
    }

    info!("{}", cube);
    Some(algorithm.expand().shorten())
}
