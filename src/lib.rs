use wasm_bindgen::prelude::*;

pub mod entities;
pub mod read;
pub mod solver;
pub mod util;

#[wasm_bindgen]
pub fn solve(
    input: &str,
    allow_ops: &str,
    max_depth: usize,
    num: usize,
    solve_by_roux: bool,
) -> String {
    let (init, goal) = read::read(input);
    if solve_by_roux {
        if let Some(alg) = solver::roux(&init, false) {
            format!("{}", alg)
        } else {
            String::from("(failed:no_solutions)")
        }
    } else {
        match read::parse_ops(allow_ops) {
            Ok((_, allow_ops)) => {
                let solutions = solver::search(&init, &goal, allow_ops.data, max_depth, num, false);
                if solutions.is_empty() {
                    String::from("(failed:no_solutions)")
                } else {
                    solutions
                        .into_iter()
                        .map(|ops| format!("{}", ops))
                        .collect::<Vec<String>>()
                        .join(";")
                }
            }
            Err(_) => String::from("(failed:invalid_operations)"),
        }
    }
}

#[cfg(test)]
mod test_operation {

    use crate::solve;

    #[test]
    fn test_solve_scramble() {
        let input = "
            Scramble {
              DDU'
            }
        ";
        let allow_ops = "D U";
        let solutions = solve(&input, &allow_ops, false);
        assert_ne!(solutions, String::from(""));
        assert!(!solutions.as_str().starts_with("(failed"));
    }

    #[test]
    fn test_solve_roux() {
        let input = "
            Scramble {
              U' F B
            }
        ";
        let solutions = solve(&input, "", true);
        assert_ne!(solutions, String::from(""));
        assert!(!solutions.as_str().starts_with("(failed"));
    }
}
