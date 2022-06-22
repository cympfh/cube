use crate::entities::Color;
use crate::rotate;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Face {
    data: Vec<Vec<Color>>,
}

impl Face {
    pub fn from(lines: &Vec<&str>) -> Self {
        let mut data = vec![vec![]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let c = Color::from(lines[i].chars().nth(j).unwrap_or('_'));
                data[i].push(c);
            }
        }
        Self { data }
    }
    pub fn show(&self, i: usize) -> String {
        self.data[i].iter().map(|c| format!("{}", c)).collect()
    }
    pub fn has_wildcard(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.data[i][j] == Color::Wildcard {
                    return true;
                }
            }
        }
        false
    }
}

impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.show(0))?;
        writeln!(f, "{}", self.show(1))?;
        write!(f, "{}", self.show(2))
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
    pub fn matched(&self, other: &Face) -> bool {
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

impl Face {
    pub fn rotate(&mut self, clockwise: bool) {
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
