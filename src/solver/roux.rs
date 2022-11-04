use crate::cube;
use crate::entities::*;
use crate::read;
use crate::solver::{search_any, search_one};
use log::info;

pub fn roux(cube: &Cube, verbose: bool) -> Option<Ops> {
    let mut cube: Cube = cube.clone();
    let mut algorithm = Ops::default();

    use Operation::*;

    info!("FB/1");
    let subgoal = cube![
        . . . ;
        . . . ;
        . . . ;
        . . . . . . . . . . . . ;
        R . . . . . . . . . B B ;
        R . . . . . . . . . B B ;
        W . . ;
        W . . ;
        . . . ;
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
    match search_one(&subcube, &subgoal, allowed_ops, 8, verbose) {
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
        . . . ;
        . . . ;
        . . . ;
        . . . . . . . . . . . . ;
        R . . . . . . . O B B B ;
        R . . . . . . . O B B B ;
        W . . ;
        W . . ;
        W . . ;
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
        RightDouble(true),
        RightDouble(false),
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

    info!("SB/1");
    let subgoal = cube![
        . . . ;
        . . . ;
        . . . ;
        . . . . . . . . . . . . ;
        R . R G G . . . O B B B ;
        R . R G G . . . O B B B ;
        W . W ;
        W . W ;
        W . . ;
    ];
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
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
    match search_one(&subcube, &subgoal, allowed_ops, 8, verbose) {
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
        . . . ;
        . . . ;
        . . . ;
        . . . . . . . . . . . . ;
        R . R G G G O . O B B B ;
        R . R G G G O . O B B B ;
        W . W ;
        W . W ;
        W . W ;
    ];
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
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
    match search_one(&subcube, &subgoal, allowed_ops, 8, verbose) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("CMLL");
    let subgoal = cube![
        Y . Y ;
        . . . ;
        Y . Y ;
        R . R G . G O . O B . B ;
        R . R G G G O . O B B B ;
        R . R G G G O . O B B B ;
        W . W ;
        W . W ;
        W . W ;
    ];
    let mut subcube = cube.clone();
    subcube.mask(&subgoal);
    let sexy = Operation::Compound(
        "Sx".to_string(),
        true,
        read::parse_ops(&"RUR'U'").unwrap().1.data,
    );
    let ways = vec![
        (vec![Up(true), Up(false), Right(true), Right(false)], 8),
        (
            vec![
                Up(true),
                Up(false),
                sexy.clone(),
                sexy.rev(),
                Front(true),
                Front(false),
            ],
            6,
        ),
        (
            vec![
                Front(true),
                Front(false),
                Up(true),
                Up(false),
                Right(true),
                Right(false),
            ],
            8,
        ),
        (
            vec![
                Front(true),
                Front(false),
                Up(true),
                Up(false),
                Down(true),
                Down(false),
                Right(true),
                Right(false),
            ],
            8,
        ),
    ];
    match search_any(&subcube, &subgoal, ways, verbose, 20) {
        Some(alg) => {
            algorithm.extend(&alg);
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

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
            cube = alg.apply(&cube);
        }
        None => {
            return None;
        }
    }

    info!("{}", cube);
    Some(algorithm.shorten())
}
