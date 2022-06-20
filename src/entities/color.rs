#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
    Wildcard,
}

impl Color {
    pub fn from(c: char) -> Self {
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
