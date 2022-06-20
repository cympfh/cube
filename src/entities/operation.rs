/// https://tribox.com/3x3x3/solution/notation/
/// Omit: E and S
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
                X(true) => "X",
                X(false) => "X'",
                Y(true) => "Y",
                Y(false) => "Y'",
                Z(true) => "Z",
                Z(false) => "Z'",
            }
        )
    }
}
