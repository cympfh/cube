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
    pub fn new(front: Face, back: Face, up: Face, down: Face, left: Face, right: Face) -> Self {
        Self {
            front,
            back,
            up,
            down,
            left,
            right,
        }
    }
    pub fn from(lines: Vec<Vec<Color>>) -> Self {
        let up = Face::from(lines[0..3].to_vec());
        let front = Face::from(
            lines[3..6]
                .iter()
                .map(|line| line[..3].to_vec())
                .collect::<Vec<_>>(),
        );
        let right = Face::from(
            lines[3..6]
                .iter()
                .map(|line| line[3..6].to_vec())
                .collect::<Vec<_>>(),
        );
        let back = Face::from(
            lines[3..6]
                .iter()
                .map(|line| line[6..9].to_vec())
                .collect::<Vec<_>>(),
        );
        let left = Face::from(
            lines[3..6]
                .iter()
                .map(|line| line[9..].to_vec())
                .collect::<Vec<_>>(),
        );
        let down = Face::from(lines[6..9].to_vec());
        Self {
            front,
            back,
            up,
            down,
            left,
            right,
        }
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
        writeln!(f, "{{")?;
        for i in 0..3 {
            writeln!(f, "  {}", self.up.show(i))?;
        }
        for i in 0..3 {
            writeln!(
                f,
                "  {} {} {} {}",
                self.front.show(i),
                self.right.show(i),
                self.back.show(i),
                self.left.show(i),
            )?;
        }
        for i in 0..3 {
            writeln!(f, "  {}", self.down.show(i))?;
        }
        write!(f, "}}")
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
            UpDouble(clockwise) => {
                self.apply(Up(clockwise));
                self.apply(Equator(!clockwise));
            }
            DownDouble(clockwise) => {
                self.apply(Down(clockwise));
                self.apply(Equator(clockwise));
            }
            FrontDouble(clockwise) => {
                self.apply(Front(clockwise));
                self.apply(Standing(clockwise));
            }
            BackDouble(clockwise) => {
                self.apply(Back(clockwise));
                self.apply(Standing(!clockwise));
            }
            LeftDouble(clockwise) => {
                self.apply(Left(clockwise));
                self.apply(Middle(clockwise));
            }
            RightDouble(clockwise) => {
                self.apply(Right(clockwise));
                self.apply(Middle(!clockwise));
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
    use crate::entities::*;
    use crate::{cube, Cube, Operation, Ops};
    use Operation::*;

    #[test]
    fn test_simple_operation() {
        let solved_cube = cube![
            Y Y Y;
            Y Y Y;
            Y Y Y;
            R R R G G G O O O B B B;
            R R R G G G O O O B B B;
            R R R G G G O O O B B B;
            W W W;
            W W W;
            W W W;
        ];
        {
            let mut c = solved_cube.clone();
            c.apply(Front(true));
            let d = cube![
                Y Y Y;
                Y Y Y;
                B B B;
                R R R Y G G O O O B B W;
                R R R Y G G O O O B B W;
                R R R Y G G O O O B B W;
                G G G;
                W W W;
                W W W;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Back(true));
            let d = cube![
                G G G ;
                Y Y Y ;
                Y Y Y ;
                R R R G G W O O O Y B B ;
                R R R G G W O O O Y B B ;
                R R R G G W O O O Y B B ;
                W W W ;
                W W W ;
                B B B ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Right(true));
            c.apply(Back(true));
            let d = cube![
                G G G ;
                Y Y R ;
                Y Y R ;
                R R W G G O Y Y Y R B B ;
                R R W G G W O O O Y B B ;
                R R W G G W O O O Y B B ;
                W W O ;
                W W O ;
                B B B ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Front(true));
            c.apply(Left(true));
            let d = cube![
                O Y Y ;
                O Y Y ;
                O B B ;
                Y R R Y G G O O W B B B ;
                Y R R Y G G O O W B B B ;
                B R R Y G G O O G W W W ;
                R G G ;
                R W W ;
                R W W ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Left(true));
            c.apply(Down(true));
            let d = cube![
                O Y Y ;
                O Y Y ;
                O Y Y ;
                Y R R G G G O O W B B B ;
                Y R R G G G O O W B B B ;
                B B B Y R R G G G O O W ;
                R R R ;
                W W W ;
                W W W ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Z(true));
            let d = cube![
                B B B ;
                B B B ;
                B B B ;
                R R R Y Y Y O O O W W W ;
                R R R Y Y Y O O O W W W ;
                R R R Y Y Y O O O W W W ;
                G G G ;
                G G G ;
                G G G ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = solved_cube.clone();
            c.apply(Z(true));
            c.apply(X(true));
            c.apply(Z(false));
            let d = cube![
                Y Y Y ;
                Y Y Y ;
                Y Y Y ;
                G G G O O O B B B R R R ;
                G G G O O O B B B R R R ;
                G G G O O O B B B R R R ;
                W W W ;
                W W W ;
                W W W ;
            ];
            assert_eq!(c, d);
        }
    }

    #[test]
    fn test_one_operation() {
        // Scrambled with M'U'M'U'F
        let c = cube![
            Y R Y ;
            B W W ;
            B B G ;
            R R O Y O B R W R G Y W ;
            O O Y R G G O R O B B Y ;
            R R O Y G G O Y O B B W ;
            G G B ;
            W Y W ;
            W G W ;
        ];
        {
            let mut c = c.clone();
            c.apply(X(true));
            let d = cube![
                R R O ;
                O O Y ;
                R R O ;
                G G B Y R Y G B B W Y W ;
                W Y W G G O W W B Y B B ;
                W G W G G B Y R Y G B B ;
                O Y O ;
                O R O ;
                R W R ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = c.clone();
            c.apply(Y(true));
            let d = cube![
                B B Y ;
                B W R ;
                G W Y ;
                Y O B R W R G Y W R R O ;
                R G G O R O B B Y O O Y ;
                Y G G O Y O B B W R R O ;
                B W W ;
                G Y G ;
                G W W ;
            ];
            assert_eq!(c, d);
        }
        {
            let mut c = c.clone();
            c.apply(Z(true));
            let d = cube![
                B B G ;
                B B Y ;
                W Y W ;
                R O R B B Y R O O W W G ;
                R O R B W R W R Y G Y G ;
                O Y O G W Y R O O W W B ;
                Y R Y ;
                G G O ;
                G G B ;
            ];
            assert_eq!(c, d);
        }
    }

    #[test]
    fn test_canceling() {
        let solved_cube = cube![
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
        let c = cube![
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
            let d = cube![
                Y Y Y ;
                Y Y Y ;
                Y Y Y ;
                R G G O R R G O O B B B ;
                R R R G G G O O O B B B ;
                R R R G G G O O O B B B ;
                W W W ;
                W W W ;
                W W W ;
            ];
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
            let d = cube![
                Y Y Y ;
                Y Y Y ;
                Y Y Y ;
                R G R G R G O B O B O B ;
                R R R G G G O O O B B B ;
                R R R G G G O O O B B B ;
                W W W ;
                W W W ;
                W W W ;
            ];
            assert!(c.matched(&d));
        }
    }
    #[test]
    fn test_f2l() {
        let mut c = cube![
            * * * ;
            * * * ;
            * * * ;
            * * * * * * * * * * * * ;
            R R G R G G O O O B B B ;
            R R R G G G O O O B B B ;
            W W W ;
            W W W ;
            W W W ;
        ];
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
        let d = cube![
            * * * ;
            * * * ;
            * * * ;
            * * * * * * * * * * * * ;
            R R R G G G O O O B B B ;
            R R R G G G O O O B B B ;
            W W W ;
            W W W ;
            W W W ;
        ];
        assert!(c.matched(&d));
    }

    #[test]
    fn test_count() {
        let c = cube![
            * * * ;
            * . * ;
            * * * ;
            * * * * * * * * * * * * ;
            R R R G G G O O O B B B ;
            R R R G G G O O O B B B ;
            W W W ;
            W W W ;
            W W W ;
        ];
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
