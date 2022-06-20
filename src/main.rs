mod entities;
mod util;
use crate::entities::{Cube, Operation};

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
    let ops = vec![X(true), Middle(false), Up(false), Right(true)];
    for op in ops {
        print!("{}", op);
    }
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
