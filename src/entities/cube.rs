use crate::entities::{Face, Operation};
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
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.up)?;
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
                        self.down[(2, 2)],
                        self.down[(2, 1)],
                        self.down[(2, 0)],
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
                        self.left[(0, 1)],
                        self.left[(1, 1)],
                        self.left[(2, 1)],
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
