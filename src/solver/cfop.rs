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

fn search_any(
    init_state: &Cube,
    goal: &Cube,
    ways: Vec<(Vec<Operation>, usize)>,
    verbose: bool,
    better_length: usize,
) -> Option<Ops> {
    let mut min_length = 999;
    let mut ret = None;
    for (allowed_ops, max_depth) in ways {
        let algs = search(init_state, goal, allowed_ops, max_depth, 1, verbose);
        if let Some(alg) = algs.get(0).map(|alg| alg.expand().shorten()) {
            if alg.len() <= better_length {
                return Some(alg);
            }
            if alg.len() < min_length {
                min_length = alg.len();
                ret = Some(alg);
            }
        }
    }
    ret
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
        Middle(true),
        Middle(false),
    ];
    match search_one(&subcube, &subgoal, allowed_ops, 4, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("F2L/#1+2");
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
            match search_one(&subcube, &subgoal, allowed_ops, 6, verbose) {
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
        read::parse_ops(&"RUR'F' RUR'U' R'FR2U'R'").unwrap().1.data,
    );
    let ja = Operation::Compound(
        "Ja".to_string(),
        true,
        read::parse_ops(&"r2FrU2 R'FRU F'r").unwrap().1.data,
    );
    let f = Operation::Compound(
        "F".to_string(),
        true,
        read::parse_ops(&"R'U'F'R UR'U'R' FR2U'R' U'RUR'UR")
            .unwrap()
            .1
            .data,
    );
    let v = Operation::Compound(
        "V".to_string(),
        true,
        read::parse_ops(&"F'UF'U 'R'F'R2 U'R'UR' FRF")
            .unwrap()
            .1
            .data,
    );
    let gb = Operation::Compound(
        "Gb".to_string(),
        true,
        read::parse_ops(&"R' U' R U D' R2 U R' U R U' R U' R2 D")
            .unwrap()
            .1
            .data,
    );
    let gd = Operation::Compound(
        "Gd".to_string(),
        true,
        read::parse_ops(&"R U R' U' D R2 U' R U' R' U R' U R2 D'")
            .unwrap()
            .1
            .data,
    );
    let na = Operation::Compound(
        "Na".to_string(),
        true,
        read::parse_ops(&"z U R' D R2 U' R D' U R' D R2 U' R D' z'")
            .unwrap()
            .1
            .data,
    );
    let nb = Operation::Compound(
        "Nb".to_string(),
        true,
        read::parse_ops(&"R' U R U' R' F' U' F R U R' F R' F' R U' R")
            .unwrap()
            .1
            .data,
    );
    let aa = Operation::Compound(
        "Aa".to_string(),
        true,
        read::parse_ops(&"x L2 D2 L' U' L D2 L' U L' x'")
            .unwrap()
            .1
            .data,
    );
    let ra = Operation::Compound(
        "Ra".to_string(),
        true,
        read::parse_ops(&"R U' R' U' R U R D R' U' R D' R' U2 R'")
            .unwrap()
            .1
            .data,
    );
    let rb = Operation::Compound(
        "Rb".to_string(),
        true,
        read::parse_ops(&"R2 F R U R U' R' F' R U2 R' U2 R")
            .unwrap()
            .1
            .data,
    );
    let ways = vec![
        (vec![Up(true), Up(false), Middle(true), Middle(false)], 7),
        (vec![Up(true), Up(false), f], 4),
        (vec![Up(true), Up(false), v], 4),
        (vec![Up(true), Up(false), jb], 5),
        (vec![Up(true), Up(false), ja], 4),
        (vec![Up(true), Up(false), gb], 4),
        (vec![Up(true), Up(false), gd], 4),
        (vec![Up(true), Up(false), na], 4),
        (vec![Up(true), Up(false), nb], 4),
        (vec![Up(true), Up(false), aa], 4),
        (vec![Up(true), Up(false), ra], 4),
        (vec![Up(true), Up(false), rb], 4),
        (
            vec![
                Up(true),
                Up(false),
                sexy.clone(),
                sexy.rev(),
                sledgehammer.clone(),
                sledgehammer.rev(),
            ],
            7,
        ),
    ];
    match search_any(&cube, &subgoal, ways, verbose, 23) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("{}", cube);
    Some(algorithm.expand().shorten())
}
