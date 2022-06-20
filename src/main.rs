#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
    Wildcard,
}
impl Color {
    fn from(c: char) -> Self {
        use Color::*;
        match c.to_lowercase().next().unwrap_or('_') {
            'w' => White,
            'y' => Yellow,
            'r' => Red,
            'o' => Orange,
            'b' => Blue,
            'g' => Green,
            '*' => Wildcard,
            _ => panic!("Unknown color: {}", c),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        write!(
            f,
            "{}",
            match self {
                White => 'W',
                Yellow => 'Y',
                Red => 'R',
                Orange => 'O',
                Blue => 'B',
                Green => 'G',
                _ => '*',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Face {
    data: Vec<Vec<Color>>,
}
impl Face {
    fn from(lines: &Vec<&str>) -> Self {
        let mut data = vec![vec![]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let c = Color::from(lines[i].chars().nth(j).unwrap_or('_'));
                data[i].push(c);
            }
        }
        Self { data }
    }
    fn show(&self, i: usize) -> String {
        self.data[i].iter().map(|c| format!("{}", c)).collect()
    }
}
impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.show(0))?;
        writeln!(f, "{}", self.show(1))?;
        writeln!(f, "{}", self.show(2))
    }
}

impl std::ops::Index<(usize, usize)> for Face {
    type Output = Color;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Face {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut Color {
        &mut self.data[index.0][index.1]
    }
}
impl Face {
    fn matched(&self, other: &Face) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self[(i, j)] != Color::Wildcard
                    && other[(i, j)] != Color::Wildcard
                    && self[(i, j)] != other[(i, j)]
                {
                    return false;
                }
            }
        }
        true
    }
}

macro_rules! rotate {
    ($shift:expr, [ $( $loc:expr ),* $(,)? ]) => {
        let mut colors = vec![ $( $loc ),* ];
        colors.rotate_right($shift);
        rotate!(@assign, colors, 0, $( $loc ),* );
    };
    (@assign, $colors:expr, $index:expr $(,)? ) => {};
    (@assign, $colors:expr, $index:expr, $loc:expr $(,)?) => {
        $loc = $colors[$index];
    };
    (@assign, $colors:expr, $index:expr, $loc:expr, $( $rest:expr ),*) => {
        $loc = $colors[$index];
        rotate!(@assign, $colors, ($index + 1), $( $rest ),*);
    };
}

impl Face {
    fn rotate(&mut self, clockwise: bool) {
        // corners
        rotate!(
            if clockwise { 1 } else { 3 },
            [self[(0, 0)], self[(0, 2)], self[(2, 2)], self[(2, 0)]]
        );
        // edges
        rotate!(
            if clockwise { 1 } else { 3 },
            [self[(0, 1)], self[(1, 2)], self[(2, 1)], self[(1, 0)]]
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    front: Face,
    back: Face,
    up: Face,
    down: Face,
    left: Face,
    right: Face,
}
impl Cube {
    fn from(lines: &Vec<&str>) -> Self {
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
    fn read() -> Self {
        let stdin = std::io::stdin();
        let mut line = String::new();
        let mut buffer = vec![];
        while buffer.len() < 9 {
            while line.is_empty() {
                let _ = stdin.read_line(&mut line);
                line = line.trim().to_string();
            }
            buffer.push(line.clone());
            line.clear();
        }
        Cube::from(&buffer.iter().map(|s| s.as_str()).collect())
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

/// https://tribox.com/3x3x3/solution/notation/
/// Omit: E and S
enum Operation {
    Up(bool),
    Down(bool),
    Left(bool),
    Right(bool),
    Front(bool),
    Back(bool),
    Middle(bool),
    Equator(bool),
    Standing(bool),
    X(bool),
    Y(bool),
    Z(bool),
}

impl Cube {
    fn apply(&mut self, op: Operation) {
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

impl Cube {
    fn matched(&self, other: &Cube) -> bool {
        self.up.matched(&other.up)
            && self.down.matched(&other.down)
            && self.right.matched(&other.right)
            && self.left.matched(&other.left)
            && self.front.matched(&other.front)
            && self.back.matched(&other.back)
    }
}

fn main() {
    let mut cube = Cube::read();
    let goal = Cube::read();
    use Operation::*;
    println!("Init:\n{}", &cube);
    println!("Goal:\n{}", &goal);
    cube.apply(Up(false));
    cube.apply(Down(true));
    cube.apply(Down(true));
    println!("{}", &cube);
    println!("{}", cube.matched(&goal));
}

#[cfg(test)]
mod test {
    use crate::{Cube, Operation};
    use Operation::*;
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
}
