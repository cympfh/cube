use crate::entities::Cube;

/// https://tribox.com/3x3x3/solution/notation/
/// Omit: E and S
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Up(bool), // clockwise?
    Down(bool),
    Front(bool),
    Back(bool),
    Left(bool),
    Right(bool),
    UpDouble(bool),
    DownDouble(bool),
    FrontDouble(bool),
    BackDouble(bool),
    LeftDouble(bool),
    RightDouble(bool),
    Middle(bool),
    Equator(bool),
    Standing(bool),
    X(bool),
    Y(bool),
    Z(bool),
    Sexy(bool),
    SledgeHammer(bool),
    Jb(bool),
}

impl Operation {
    pub fn rev(&self) -> Self {
        use Operation::*;
        match self {
            Up(clockwise) => Up(!clockwise),
            Down(clockwise) => Down(!clockwise),
            Left(clockwise) => Left(!clockwise),
            Right(clockwise) => Right(!clockwise),
            Front(clockwise) => Front(!clockwise),
            Back(clockwise) => Back(!clockwise),
            UpDouble(clockwise) => UpDouble(!clockwise),
            DownDouble(clockwise) => DownDouble(!clockwise),
            LeftDouble(clockwise) => LeftDouble(!clockwise),
            RightDouble(clockwise) => RightDouble(!clockwise),
            FrontDouble(clockwise) => FrontDouble(!clockwise),
            BackDouble(clockwise) => BackDouble(!clockwise),
            Middle(clockwise) => Middle(!clockwise),
            Equator(clockwise) => Equator(!clockwise),
            Standing(clockwise) => Standing(!clockwise),
            X(clockwise) => X(!clockwise),
            Y(clockwise) => Y(!clockwise),
            Z(clockwise) => Z(!clockwise),
            Sexy(clockwise) => Sexy(!clockwise),
            SledgeHammer(clockwise) => SledgeHammer(!clockwise),
            Jb(clockwise) => Jb(!clockwise),
        }
    }
    pub fn is_reversed(&self) -> bool {
        use Operation::*;
        match self {
            Up(clockwise) => !clockwise,
            Down(clockwise) => !clockwise,
            Left(clockwise) => !clockwise,
            Right(clockwise) => !clockwise,
            Front(clockwise) => !clockwise,
            Back(clockwise) => !clockwise,
            UpDouble(clockwise) => !clockwise,
            DownDouble(clockwise) => !clockwise,
            LeftDouble(clockwise) => !clockwise,
            RightDouble(clockwise) => !clockwise,
            FrontDouble(clockwise) => !clockwise,
            BackDouble(clockwise) => !clockwise,
            Middle(clockwise) => !clockwise,
            Equator(clockwise) => !clockwise,
            Standing(clockwise) => !clockwise,
            X(clockwise) => !clockwise,
            Y(clockwise) => !clockwise,
            Z(clockwise) => !clockwise,
            Sexy(clockwise) => !clockwise,
            SledgeHammer(clockwise) => !clockwise,
            Jb(clockwise) => !clockwise,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operation::*;
        write!(
            f,
            "{}",
            match *self {
                Up(true) => "U",
                Up(false) => "U'",
                Down(true) => "D",
                Down(false) => "D'",
                Front(true) => "F",
                Front(false) => "F'",
                Back(true) => "B",
                Back(false) => "B'",
                Left(true) => "L",
                Left(false) => "L'",
                Right(true) => "R",
                Right(false) => "R'",
                UpDouble(true) => "u",
                UpDouble(false) => "u'",
                DownDouble(true) => "d",
                DownDouble(false) => "d'",
                FrontDouble(true) => "f",
                FrontDouble(false) => "f'",
                BackDouble(true) => "b",
                BackDouble(false) => "b'",
                LeftDouble(true) => "l",
                LeftDouble(false) => "l'",
                RightDouble(true) => "r",
                RightDouble(false) => "r'",
                Middle(true) => "M",
                Middle(false) => "M'",
                Equator(true) => "E",
                Equator(false) => "E'",
                Standing(true) => "S",
                Standing(false) => "S'",
                X(true) => "x",
                X(false) => "x'",
                Y(true) => "y",
                Y(false) => "y'",
                Z(true) => "z",
                Z(false) => "z'",
                Sexy(true) => "(Sx)",
                Sexy(false) => "(Sx)'",
                SledgeHammer(true) => "(SH)",
                SledgeHammer(false) => "(SH)'",
                Jb(true) => "(Jb)",
                Jb(false) => "(Jb)'",
            }
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ops {
    pub data: Vec<Operation>,
}

impl Ops {
    #[allow(dead_code)]
    pub fn new(data: Vec<Operation>) -> Self {
        Self { data }
    }
    pub fn push(&mut self, op: Operation) {
        self.data.push(op);
    }
    pub fn pop(&mut self) -> Option<Operation> {
        self.data.pop()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// length of expanded
    pub fn weight(&self) -> usize {
        use Operation::*;
        self.data
            .iter()
            .map(|op| match op {
                Sexy(_) => 4,
                SledgeHammer(_) => 4,
                Jb(_) => 14,
                _ => 1,
            })
            .sum::<usize>()
    }
    pub fn last(&self) -> Option<Operation> {
        self.data.last().cloned()
    }
    /// if the last 2 operations are same, dont repeat it.
    pub fn last_repeat(&self) -> Option<Operation> {
        let n = self.data.len();
        if n >= 2 && self.data[n - 2] == self.data[n - 1] {
            Some(self.data[n - 1])
        } else {
            None
        }
    }
    pub fn rev(&self) -> Self {
        let mut reversed = vec![];
        for &op in self.data.iter().rev() {
            reversed.push(op.rev());
        }
        Self { data: reversed }
    }
    pub fn extend(&mut self, other: &Ops) {
        for &op in other.data.iter() {
            self.data.push(op);
        }
    }
    pub fn apply(&self, cube: &Cube) -> Cube {
        let mut c = cube.clone();
        for &op in self.data.iter() {
            c.apply(op);
        }
        c
    }
    pub fn expand(&self) -> Self {
        use Operation::*;
        let mut ops = Ops::default();
        for &op in self.data.iter() {
            match op {
                Sexy(true) => {
                    ops.push(Right(true));
                    ops.push(Up(true));
                    ops.push(Right(false));
                    ops.push(Up(false));
                }
                Sexy(false) => {
                    ops.push(Up(true));
                    ops.push(Right(true));
                    ops.push(Up(false));
                    ops.push(Right(false));
                }
                SledgeHammer(true) => {
                    ops.push(Right(false));
                    ops.push(Front(true));
                    ops.push(Right(true));
                    ops.push(Front(false));
                }
                SledgeHammer(false) => {
                    ops.push(Front(true));
                    ops.push(Right(false));
                    ops.push(Front(false));
                    ops.push(Right(true));
                }
                Jb(true) => {
                    // R U R' F' R U R' U' R' F R2 U' R'
                    for op in [
                        Right(true),
                        Up(true),
                        Right(false),
                        Front(false),
                        Right(true),
                        Up(true),
                        Right(false),
                        Up(false),
                        Right(false),
                        Front(true),
                        Right(true),
                        Right(true),
                        Up(false),
                        Right(false),
                    ] {
                        ops.push(op);
                    }
                }
                Jb(false) => {
                    // (R U R' F' R U R' U' R' F R2 U' R')'
                    for op in [
                        Right(true),
                        Up(true),
                        Right(false),
                        Front(false),
                        Right(true),
                        Up(true),
                        Right(false),
                        Up(false),
                        Right(false),
                        Front(true),
                        Right(true),
                        Right(true),
                        Up(false),
                        Right(false),
                    ]
                    .iter()
                    .rev()
                    {
                        ops.push(op.rev());
                    }
                }
                _ => ops.push(op),
            }
        }
        ops
    }
    pub fn shorten(&self) -> Self {
        let mut ops = Ops::default();
        for &op in self.data.iter() {
            ops.push(op);
            let m = ops.len();
            if m >= 3 {
                let a = ops.data[m - 1];
                let b = ops.data[m - 2];
                let c = ops.data[m - 3];
                if a == b && a == c {
                    ops.pop();
                    ops.pop();
                    ops.pop();
                    ops.push(a.rev());
                }
            }
            let m = ops.len();
            if m >= 2 {
                let a = ops.data[m - 1];
                let b = ops.data[m - 2];
                if a == b.rev() {
                    ops.pop();
                    ops.pop();
                }
            }
        }
        ops
    }
}

impl std::fmt::Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            write!(f, "(nop)")
        } else {
            for op in self.data.iter() {
                write!(f, "{}", op)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test_operation {
    use crate::entities::{Operation, Ops};
    use Operation::*;

    #[test]
    fn test_operation() {
        let u = Up(true);
        assert!(!u.is_reversed());
        let u_prime = Up(false);
        assert!(u_prime.is_reversed());
    }

    #[test]
    fn test_ops() {
        let mut ops = Ops::default();
        assert_eq!(ops.last(), None);
        assert_eq!(ops.last_repeat(), None);

        ops.push(Up(true));
        assert_eq!(ops.last(), Some(Up(true)));
        assert_eq!(ops.last_repeat(), None);

        ops.push(Up(true));
        assert_eq!(ops.last(), Some(Up(true)));
        assert_eq!(ops.last_repeat(), Some(Up(true)));

        ops.push(Up(false));
        assert_eq!(ops.last(), Some(Up(false)));
        assert_eq!(ops.last_repeat(), None);
    }
}
