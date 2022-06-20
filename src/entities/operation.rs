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
