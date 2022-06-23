use crate::entities::Cube;

/// https://tribox.com/3x3x3/solution/notation/
/// Omit: E and S
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
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

impl Operation {
    pub fn rev(&self) -> Self {
        use Operation::*;
        match self {
            Up(ccw) => Up(!ccw),
            Down(ccw) => Down(!ccw),
            Left(ccw) => Left(!ccw),
            Right(ccw) => Right(!ccw),
            Front(ccw) => Front(!ccw),
            Back(ccw) => Back(!ccw),
            Middle(ccw) => Middle(!ccw),
            Equator(ccw) => Equator(!ccw),
            Standing(ccw) => Standing(!ccw),
            X(ccw) => X(!ccw),
            Y(ccw) => Y(!ccw),
            Z(ccw) => Z(!ccw),
        }
    }
    pub fn is_reversed(&self) -> bool {
        use Operation::*;
        match self {
            Up(ccw) => !ccw,
            Down(ccw) => !ccw,
            Left(ccw) => !ccw,
            Right(ccw) => !ccw,
            Front(ccw) => !ccw,
            Back(ccw) => !ccw,
            Middle(ccw) => !ccw,
            Equator(ccw) => !ccw,
            Standing(ccw) => !ccw,
            X(ccw) => !ccw,
            Y(ccw) => !ccw,
            Z(ccw) => !ccw,
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
            }
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ops {
    data: Vec<Operation>,
}

impl Ops {
    #[allow(dead_code)]
    pub fn new(data: Vec<Operation>) -> Self {
        Self { data }
    }
    pub fn push(&mut self, op: Operation) {
        self.data.push(op);
    }
    pub fn len(&self) -> usize {
        self.data.len()
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
