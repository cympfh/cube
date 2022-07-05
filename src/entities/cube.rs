use crate::entities::{Color, Face, Operation};
use crate::rotate;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cube {
    pub front: Face,
    pub back: Face,
    pub up: Face,
    pub down: Face,
    pub left: Face,
    pub right: Face,
}

impl Cube {
    pub fn from(lines: &Vec<&str>) -> Self {
        let up = Face::from(&lines[0..3].to_vec());
        let front = Face::from(
            &lines[3..6]
                .iter()
                .map(|&line| &line[..3])
                .collect::<Vec<_>>(),
        );
        let right = Face::from(
            &lines[3..6]
                .iter()
                .map(|&line| &line[3..6])
                .collect::<Vec<_>>(),
        );
        let back = Face::from(
            &lines[3..6]
                .iter()
                .map(|&line| &line[6..9])
                .collect::<Vec<_>>(),
        );
        let left = Face::from(
            &lines[3..6]
                .iter()
                .map(|&line| &line[9..])
                .collect::<Vec<_>>(),
        );
        let down = Face::from(&lines[6..9].to_vec());
        Self {
            front,
            back,
            up,
            down,
            left,
            right,
        }
    }
    pub fn read() -> Self {
        let stdin = std::io::stdin();
        let mut buffer = vec![];
        while buffer.len() < 9 {
            let mut line = String::new();
            while line.is_empty() {
                let _ = stdin.read_line(&mut line);
                line = line.trim().to_string();
                if line.starts_with('#') {
                    line.clear();
                }
            }
            buffer.push(line.clone());
        }
        Cube::from(&buffer.iter().map(|s| s.as_str()).collect())
    }
    pub fn matched(&self, other: &Cube) -> bool {
        self.up.matched(&other.up)
            && self.down.matched(&other.down)
            && self.right.matched(&other.right)
            && self.left.matched(&other.left)
            && self.front.matched(&other.front)
            && self.back.matched(&other.back)
    }
    pub fn has_wildcard(&self) -> bool {
        self.up.has_wildcard()
            || self.down.has_wildcard()
            || self.right.has_wildcard()
            || self.left.has_wildcard()
            || self.front.has_wildcard()
            || self.back.has_wildcard()
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.up)?;
        for i in 0..3 {
            write!(f, "{}", self.front.show(i))?;
            write!(f, "{}", self.right.show(i))?;
            write!(f, "{}", self.back.show(i))?;
            writeln!(f, "{}", self.left.show(i))?;
        }
        write!(f, "{}", self.down)
    }
}

impl Cube {
    pub fn apply(&mut self, op: Operation) {
        use Operation::*;
        match op {
            Up(clockwise) => {
                self.up.rotate(clockwise);
                rotate!(
                    if clockwise { 9 } else { 3 },
                    [
                        self.front[(0, 0)],
                        self.front[(0, 1)],
                        self.front[(0, 2)],
                        self.right[(0, 0)],
                        self.right[(0, 1)],
                        self.right[(0, 2)],
                        self.back[(0, 0)],
                        self.back[(0, 1)],
                        self.back[(0, 2)],
                        self.left[(0, 0)],
                        self.left[(0, 1)],
                        self.left[(0, 2)],
                    ]
                );
            }
            Down(clockwise) => {
                self.down.rotate(clockwise);
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.front[(2, 0)],
                        self.front[(2, 1)],
                        self.front[(2, 2)],
                        self.right[(2, 0)],
                        self.right[(2, 1)],
                        self.right[(2, 2)],
                        self.back[(2, 0)],
                        self.back[(2, 1)],
                        self.back[(2, 2)],
                        self.left[(2, 0)],
                        self.left[(2, 1)],
                        self.left[(2, 2)],
                    ]
                );
            }
            Front(clockwise) => {
                self.front.rotate(clockwise);
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.up[(2, 0)],
                        self.up[(2, 1)],
                        self.up[(2, 2)],
                        self.right[(0, 0)],
                        self.right[(1, 0)],
                        self.right[(2, 0)],
                        self.down[(0, 2)],
                        self.down[(0, 1)],
                        self.down[(0, 0)],
                        self.left[(2, 2)],
                        self.left[(1, 2)],
                        self.left[(0, 2)],
                    ]
                );
            }
            Back(clockwise) => {
                self.back.rotate(clockwise);
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.up[(0, 2)],
                        self.up[(0, 1)],
                        self.up[(0, 0)],
                        self.left[(0, 0)],
                        self.left[(1, 0)],
                        self.left[(2, 0)],
                        self.down[(2, 0)],
                        self.down[(2, 1)],
                        self.down[(2, 2)],
                        self.right[(2, 2)],
                        self.right[(1, 2)],
                        self.right[(0, 2)],
                    ]
                );
            }
            Right(clockwise) => {
                self.right.rotate(clockwise);
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.up[(2, 2)],
                        self.up[(1, 2)],
                        self.up[(0, 2)],
                        self.back[(0, 0)],
                        self.back[(1, 0)],
                        self.back[(2, 0)],
                        self.down[(2, 2)],
                        self.down[(1, 2)],
                        self.down[(0, 2)],
                        self.front[(2, 2)],
                        self.front[(1, 2)],
                        self.front[(0, 2)],
                    ]
                );
            }
            Left(clockwise) => {
                self.left.rotate(clockwise);
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.up[(0, 0)],
                        self.up[(1, 0)],
                        self.up[(2, 0)],
                        self.front[(0, 0)],
                        self.front[(1, 0)],
                        self.front[(2, 0)],
                        self.down[(0, 0)],
                        self.down[(1, 0)],
                        self.down[(2, 0)],
                        self.back[(2, 2)],
                        self.back[(1, 2)],
                        self.back[(0, 2)],
                    ]
                );
            }
            Middle(clockwise) => {
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.up[(0, 1)],
                        self.up[(1, 1)],
                        self.up[(2, 1)],
                        self.front[(0, 1)],
                        self.front[(1, 1)],
                        self.front[(2, 1)],
                        self.down[(0, 1)],
                        self.down[(1, 1)],
                        self.down[(2, 1)],
                        self.back[(2, 1)],
                        self.back[(1, 1)],
                        self.back[(0, 1)],
                    ]
                );
            }
            Equator(clockwise) => {
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.front[(1, 0)],
                        self.front[(1, 1)],
                        self.front[(1, 2)],
                        self.right[(1, 0)],
                        self.right[(1, 1)],
                        self.right[(1, 2)],
                        self.back[(1, 0)],
                        self.back[(1, 1)],
                        self.back[(1, 2)],
                        self.left[(1, 0)],
                        self.left[(1, 1)],
                        self.left[(1, 2)],
                    ]
                );
            }
            Standing(clockwise) => {
                rotate!(
                    if clockwise { 3 } else { 9 },
                    [
                        self.up[(1, 0)],
                        self.up[(1, 1)],
                        self.up[(1, 2)],
                        self.right[(0, 1)],
                        self.right[(1, 1)],
                        self.right[(2, 1)],
                        self.down[(1, 2)],
                        self.down[(1, 1)],
                        self.down[(1, 0)],
                        self.left[(2, 1)],
                        self.left[(1, 1)],
                        self.left[(0, 1)],
                    ]
                );
            }
            X(clockwise) => {
                self.apply(Right(clockwise));
                self.apply(Middle(!clockwise));
                self.apply(Left(!clockwise));
            }
            Y(clockwise) => {
                self.apply(Up(clockwise));
                self.apply(Equator(!clockwise));
                self.apply(Down(!clockwise));
            }
            Z(clockwise) => {
                self.apply(Front(clockwise));
                self.apply(Standing(clockwise));
                self.apply(Back(!clockwise));
            }
        }
    }
}

use std::collections::BTreeMap;
impl Cube {
    pub fn count(&self) -> BTreeMap<Color, usize> {
        let mut count = BTreeMap::new();
        for col in [
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
            Color::White,
            Color::Yellow,
            Color::Other,
            Color::Wildcard,
        ] {
            count.insert(col, 0);
        }
        for face in [
            &self.front,
            &self.back,
            &self.up,
            &self.down,
            &self.left,
            &self.right,
        ] {
            for i in 0..3 {
                for j in 0..3 {
                    let col = face.at(i, j);
                    count.entry(col).and_modify(|i| *i += 1).or_insert(1);
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test_cube {
    use crate::entities::Color;
    use crate::{Cube, Operation, Ops};
    use Operation::*;

    #[test]
    fn test_simple_operation() {
        let solved_cube = Cube::from(&vec![
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
        {
            let mut c = solved_cube.clone();
            c.apply(Front(true));
            let d = Cube::from(&vec![
                "YYY",
                "YYY",
                "BBB",
                "RRRYGGOOOBBW",
                "RRRYGGOOOBBW",
                "RRRYGGOOOBBW",
                "GGG",
                "WWW",
                "WWW",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Back(true));
            let d = Cube::from(&vec![
                "GGG",
                "YYY",
                "YYY",
                "RRRGGWOOOYBB",
                "RRRGGWOOOYBB",
                "RRRGGWOOOYBB",
                "WWW",
                "WWW",
                "BBB",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Right(true));
            c.apply(Back(true));
            let d = Cube::from(&vec![
                "GGG",
                "YYR",
                "YYR",
                "RRWGGOYYYRBB",
                "RRWGGWOOOYBB",
                "RRWGGWOOOYBB",
                "WWO",
                "WWO",
                "BBB",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Front(true));
            c.apply(Left(true));
            let d = Cube::from(&vec![
                "OYY",
                "OYY",
                "OBB",
                "YRRYGGOOWBBB",
                "YRRYGGOOWBBB",
                "BRRYGGOOGWWW",
                "RGG",
                "RWW",
                "RWW",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Left(true));
            c.apply(Down(true));
            let d = Cube::from(&vec![
                "OYY",
                "OYY",
                "OYY",
                "YRRGGGOOWBBB",
                "YRRGGGOOWBBB",
                "BBBYRRGGGOOW",
                "RRR",
                "WWW",
                "WWW",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Z(true));
            let d = Cube::from(&vec![
                "BBB",
                "BBB",
                "BBB",
                "RRRYYYOOOWWW",
                "RRRYYYOOOWWW",
                "RRRYYYOOOWWW",
                "GGG",
                "GGG",
                "GGG",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Z(true));
            c.apply(X(true));
            c.apply(Z(false));
            let d = Cube::from(&vec![
                "YYY",
                "YYY",
                "YYY",
                "GGGOOOBBBRRR",
                "GGGOOOBBBRRR",
                "GGGOOOBBBRRR",
                "WWW",
                "WWW",
                "WWW",
            ]);
            assert_eq!(c, d);
        }
    }

    #[test]
    fn test_one_operation() {
        // Scrambled with M'U'M'U'F
        let c = Cube::from(&vec![
            "YRY",
            "BWW",
            "BBG",
            "RROYOBRWRGYW",
            "OOYRGGOROBBY",
            "RROYGGOYOBBW",
            "GGB",
            "WYW",
            "WGW",
        ]);
        {
            let mut c = c.clone();
            c.apply(X(true));
            let d = Cube::from(&vec![
                "RRO",
                "OOY",
                "RRO",
                "GGBYRYGBBWYW",
                "WYWGGOWWBYBB",
                "WGWGGBYRYGBB",
                "OYO",
                "ORO",
                "RWR",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = c.clone();
            c.apply(Y(true));
            let d = Cube::from(&vec![
                "BBY",
                "BWR",
                "GWY",
                "YOBRWRGYWRRO",
                "RGGOROBBYOOY",
                "YGGOYOBBWRRO",
                "BWW",
                "GYG",
                "GWW",
            ]);
            assert_eq!(c, d);
        }
        {
            let mut c = c.clone();
            c.apply(Z(true));
            let d = Cube::from(&vec![
                "BBG",
                "BBY",
                "WYW",
                "RORBBYROOWWG",
                "RORBWRWRYGYG",
                "OYOGWYROOWWB",
                "YRY",
                "GGO",
                "GGB",
            ]);
            assert_eq!(c, d);
        }
    }

    #[test]
    fn test_canceling() {
        let solved_cube = Cube::from(&vec![
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
        {
            let u2 = Ops::new(vec![Up(true), Up(true)]);
            let u2_prime = Ops::new(vec![Up(false), Up(false)]);
            let c1 = u2.apply(&solved_cube);
            let c2 = u2_prime.apply(&solved_cube);
            assert_eq!(c1, c2);
        }
        {
            let r2 = Ops::new(vec![Right(true), Right(true)]);
            let r2_prime = Ops::new(vec![Right(false), Right(false)]);
            let c1 = r2.apply(&solved_cube);
            let c2 = r2_prime.apply(&solved_cube);
            assert_eq!(c1, c2);
        }
        {
            let r3 = Ops::new(vec![Right(true), Right(true), Right(true)]);
            let r_prime = Ops::new(vec![Right(false)]);
            let c1 = r3.apply(&solved_cube);
            let c2 = r_prime.apply(&solved_cube);
            assert_eq!(c1, c2);
        }
    }

    #[test]
    fn test_pll() {
        let c = Cube::from(&vec![
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
        {
            // J-perm
            let mut c = c.clone();
            c.apply(Right(true));
            c.apply(Up(true));
            c.apply(Right(false));
            c.apply(Front(false));
            c.apply(Right(true));
            c.apply(Up(true));
            c.apply(Right(false));
            c.apply(Up(false));
            c.apply(Right(false));
            c.apply(Front(true));
            c.apply(Right(true));
            c.apply(Right(true));
            c.apply(Up(false));
            c.apply(Right(false));
            c.apply(Up(false));
            let d = Cube::from(&vec![
                "YYY",
                "YYY",
                "YYY",
                "RGGORRGOOBBB",
                "RRRGGGOOOBBB",
                "RRRGGGOOOBBB",
                "WWW",
                "WWW",
                "WWW",
            ]);
            assert!(c.matched(&d));
        }
        {
            // Z-perm
            let mut c = c.clone();
            c.apply(Middle(false));
            c.apply(Up(false));
            for _ in 0..2 {
                c.apply(Middle(false));
                c.apply(Middle(false));
                c.apply(Up(false));
            }
            c.apply(Middle(false));
            c.apply(Up(false));
            c.apply(Up(false));
            c.apply(Middle(false));
            c.apply(Middle(false));
            c.apply(Up(true));
            let d = Cube::from(&vec![
                "YYY",
                "YYY",
                "YYY",
                "RGRGRGOBOBOB",
                "RRRGGGOOOBBB",
                "RRRGGGOOOBBB",
                "WWW",
                "WWW",
                "WWW",
            ]);
            assert!(c.matched(&d));
        }
    }
    #[test]
    fn test_f2l() {
        let mut c = Cube::from(&vec![
            "***",
            "***",
            "***",
            "************",
            "RRGRGGOOOBBB",
            "RRRGGGOOOBBB",
            "WWW",
            "WWW",
            "WWW",
        ]);
        c.apply(Right(false));
        c.apply(Front(true));
        c.apply(Right(true));
        c.apply(Front(false));
        c.apply(Right(true));
        c.apply(Up(false));
        c.apply(Right(false));
        c.apply(Up(true));
        c.apply(Right(true));
        c.apply(Up(false));
        c.apply(Right(false));
        c.apply(Up(false));
        c.apply(Up(false));
        c.apply(Right(true));
        c.apply(Up(false));
        c.apply(Right(false));
        let d = Cube::from(&vec![
            "***",
            "***",
            "***",
            "************",
            "RRRGGGOOOBBB",
            "RRRGGGOOOBBB",
            "WWW",
            "WWW",
            "WWW",
        ]);
        assert!(c.matched(&d));
    }

    #[test]
    fn test_count() {
        let c = Cube::from(&vec![
            "***",
            "*.*",
            "***",
            "************",
            "RRRGGGOOOBBB",
            "RRRGGGOOOBBB",
            "WWW",
            "WWW",
            "WWW",
        ]);
        let count = c.count();
        assert_eq!(count[&Color::Red], 6);
        assert_eq!(count[&Color::Green], 6);
        assert_eq!(count[&Color::Orange], 6);
        assert_eq!(count[&Color::Blue], 6);
        assert_eq!(count[&Color::White], 9);
        assert_eq!(count[&Color::Yellow], 0);
        assert_eq!(count[&Color::Other], 1);
        assert_eq!(count[&Color::Wildcard], 20);
    }
}
