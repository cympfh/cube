use crate::cube;
use crate::entities::*;
use log::error;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    combinator::{eof, map, opt, value},
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Entry {
    Init(Cube),
    Goal(Cube),
    Scramble(Ops),
}

fn cat() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    loop {
        let mut line = String::new();
        let _ = stdin.read_line(&mut line);
        if line.is_empty() {
            break;
        }
        buf.push_str(line.as_str());
    }
    buf
}

pub fn read() -> (Cube, Cube) {
    let buf = cat();
    let mut buf = buf.as_str();
    let canonical = cube![
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
    let mut entries = vec![];
    while let Ok((rest, e)) = parse_entry(buf) {
        entries.push(e);
        buf = rest;
    }
    if let Ok((rest, _)) = commentable_spaces(buf) {
        if !rest.is_empty() {
            error!("[Parse Error] {}", rest);
            panic!("Parse Error");
        }
    }
    let mut goal = canonical.clone();
    for e in entries.iter() {
        match e {
            Entry::Goal(c) => {
                goal = c.clone();
            }
            _ => {}
        }
    }
    let mut init = goal.clone();
    let mut initialized = 0;
    for e in entries.iter() {
        match e {
            Entry::Init(c) => {
                init = c.clone();
                initialized += 1;
            }
            Entry::Scramble(ops) => {
                init = ops.apply(&goal);
                initialized += 1;
            }
            _ => {}
        }
    }
    if initialized != 1 {
        error!("Init or Scramble must exist uniquely.");
        panic!("Input is invalid");
    }
    (init, goal)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((parse_init, parse_goal, parse_scramble))(input)
}

fn parse_init(input: &str) -> IResult<&str, Entry> {
    map(
        delimited(
            tuple((
                commentable_spaces,
                tag("Init"),
                commentable_spaces,
                tag("{"),
            )),
            parse_cube,
            tuple((commentable_spaces, tag("}"))),
        ),
        |cube| Entry::Init(cube),
    )(input)
}

fn parse_goal(input: &str) -> IResult<&str, Entry> {
    map(
        delimited(
            tuple((
                commentable_spaces,
                tag("Goal"),
                commentable_spaces,
                tag("{"),
            )),
            parse_cube,
            tuple((commentable_spaces, tag("}"))),
        ),
        |cube| Entry::Goal(cube),
    )(input)
}

fn parse_scramble(input: &str) -> IResult<&str, Entry> {
    map(
        delimited(
            tuple((
                commentable_spaces,
                tag("Scramble"),
                commentable_spaces,
                tag("{"),
            )),
            parse_ops,
            tuple((commentable_spaces, tag("}"))),
        ),
        |ops| Entry::Scramble(ops),
    )(input)
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    map(many1(parse_color), |colors: Vec<Color>| {
        assert!(colors.len() == 6 * 9);
        Cube::from(vec![
            colors[..3].to_vec(),
            colors[3..6].to_vec(),
            colors[6..9].to_vec(),
            colors[9..21].to_vec(),
            colors[21..33].to_vec(),
            colors[33..45].to_vec(),
            colors[45..48].to_vec(),
            colors[48..51].to_vec(),
            colors[51..54].to_vec(),
        ])
    })(input)
}

fn parse_ops(input: &str) -> IResult<&str, Ops> {
    fn parse_opss(input: &str) -> IResult<&str, Vec<Operation>> {
        use Operation::*;
        preceded(
            commentable_spaces,
            alt((
                alt((
                    value(vec![UpDouble(true), UpDouble(true)], tag("u2")),
                    value(vec![UpDouble(false)], tag("u'")),
                    value(vec![UpDouble(true)], tag("u")),
                    value(vec![DownDouble(true), DownDouble(true)], tag("d2")),
                    value(vec![DownDouble(false)], tag("d'")),
                    value(vec![DownDouble(true)], tag("d")),
                    value(vec![FrontDouble(true), FrontDouble(true)], tag("f2")),
                    value(vec![FrontDouble(false)], tag("f'")),
                    value(vec![FrontDouble(true)], tag("f")),
                    value(vec![BackDouble(true), BackDouble(true)], tag("b2")),
                    value(vec![BackDouble(false)], tag("b'")),
                    value(vec![BackDouble(true)], tag("b")),
                    value(vec![LeftDouble(true), LeftDouble(true)], tag("l2")),
                    value(vec![LeftDouble(false)], tag("l'")),
                    value(vec![LeftDouble(true)], tag("l")),
                    value(vec![RightDouble(true), RightDouble(true)], tag("r2")),
                    value(vec![RightDouble(false)], tag("r'")),
                    value(vec![RightDouble(true)], tag("r")),
                )),
                alt((
                    value(vec![UpDouble(true), UpDouble(true)], tag("Uw2")),
                    value(vec![UpDouble(false)], tag("Uw'")),
                    value(vec![UpDouble(true)], tag("Uw")),
                    value(vec![DownDouble(true), DownDouble(true)], tag("Dw2")),
                    value(vec![DownDouble(false)], tag("Dw'")),
                    value(vec![DownDouble(true)], tag("Dw")),
                    value(vec![FrontDouble(true), FrontDouble(true)], tag("Fw2")),
                    value(vec![FrontDouble(false)], tag("Fw'")),
                    value(vec![FrontDouble(true)], tag("Fw")),
                    value(vec![BackDouble(true), BackDouble(true)], tag("Bw2")),
                    value(vec![BackDouble(false)], tag("Bw'")),
                    value(vec![BackDouble(true)], tag("Bw")),
                    value(vec![LeftDouble(true), LeftDouble(true)], tag("Lw2")),
                    value(vec![LeftDouble(false)], tag("Lw'")),
                    value(vec![LeftDouble(true)], tag("Lw")),
                    value(vec![RightDouble(true), RightDouble(true)], tag("Rw2")),
                    value(vec![RightDouble(false)], tag("Rw'")),
                    value(vec![RightDouble(true)], tag("Rw")),
                )),
                alt((
                    value(vec![Up(true), Up(true)], tag("U2")),
                    value(vec![Up(false)], tag("U'")),
                    value(vec![Up(true)], tag("U")),
                    value(vec![Down(true), Down(true)], tag("D2")),
                    value(vec![Down(false)], tag("D'")),
                    value(vec![Down(true)], tag("D")),
                    value(vec![Front(true), Front(true)], tag("F2")),
                    value(vec![Front(false)], tag("F'")),
                    value(vec![Front(true)], tag("F")),
                    value(vec![Back(true), Back(true)], tag("B2")),
                    value(vec![Back(false)], tag("B'")),
                    value(vec![Back(true)], tag("B")),
                    value(vec![Left(true), Left(true)], tag("L2")),
                    value(vec![Left(false)], tag("L'")),
                    value(vec![Left(true)], tag("L")),
                    value(vec![Right(true), Right(true)], tag("R2")),
                    value(vec![Right(false)], tag("R'")),
                    value(vec![Right(true)], tag("R")),
                )),
                alt((
                    value(vec![Middle(true), Middle(true)], tag("M2")),
                    value(vec![Middle(false)], tag("M'")),
                    value(vec![Middle(true)], tag("M")),
                    value(vec![Equator(true), Equator(true)], tag("E2")),
                    value(vec![Equator(false)], tag("E'")),
                    value(vec![Equator(true)], tag("E")),
                    value(vec![Standing(true), Standing(true)], tag("S2")),
                    value(vec![Standing(false)], tag("S'")),
                    value(vec![Standing(true)], tag("S")),
                )),
                alt((
                    value(vec![X(true), X(true)], tag("x2")),
                    value(vec![X(false)], tag("x'")),
                    value(vec![X(true)], tag("x")),
                    value(vec![Y(true), Y(true)], tag("y2")),
                    value(vec![Y(false)], tag("y'")),
                    value(vec![Y(true)], tag("y")),
                    value(vec![Z(true), Z(true)], tag("z2")),
                    value(vec![Z(false)], tag("z'")),
                    value(vec![Z(true)], tag("z")),
                )),
            )),
        )(input)
    }
    let (rest, opss) = many1(parse_opss)(input)?;
    Ok((rest, Ops::new(opss.concat())))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    use Color::*;
    preceded(
        commentable_spaces,
        alt((
            value(White, tag("W")),
            value(White, tag("w")),
            value(Yellow, tag("Y")),
            value(Yellow, tag("y")),
            value(Blue, tag("B")),
            value(Blue, tag("b")),
            value(Green, tag("G")),
            value(Green, tag("g")),
            value(Red, tag("R")),
            value(Red, tag("r")),
            value(Orange, tag("O")),
            value(Orange, tag("o")),
            value(Other, tag(".")),
            value(Wildcard, tag("*")),
        )),
    )(input)
}

fn spaces(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = alt((tag("//"), tag("#"), tag(";")))(input)?;
    let (input, _) = opt(is_not("\n\r"))(input)?;
    alt((eof, spaces))(input)
}

fn commentable_spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = spaces(input)?;
    let (input, _) = many0(tuple((comment, spaces)))(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod test_read {
    use crate::cube;
    use crate::read::*;

    #[macro_export]
    macro_rules! assert_ok {
        ( $actual:expr, $expected:expr ) => {{
            assert!($actual.is_ok());
            assert_eq!($actual.unwrap().1, $expected);
        }};
    }

    #[test]
    fn test_cube() {
        assert_ok!(
            parse_cube(
                "
            YYY
            YYY
            YYY
            YYYYYYYYYYYY
            YYYYYYYYYYYY
            YYYYYYYYYYYY
            YYY
            YYY
            YYY"
            ),
            cube![
                Y Y Y;
                Y Y Y;
                Y Y Y;
                Y Y Y Y Y Y Y Y Y Y Y Y;
                Y Y Y Y Y Y Y Y Y Y Y Y;
                Y Y Y Y Y Y Y Y Y Y Y Y;
                Y Y Y;
                Y Y Y;
                Y Y Y;
            ]
        );
        assert_ok!(
            parse_cube(
                "
                    Y Y Y ;
                    Y Y Y ;
                    Y Y Y // comment
                    RRR GGG OOO BBB ;
                    RRR GGG OOO BBB ;
                    RRR GGG OOO BBB ;
                    W W W ;
                    W W W ;
                    W W W ;
                    "
            ),
            cube![
                Y Y Y;
                Y Y Y;
                Y Y Y;
                R R R G G G O O O B B B;
                R R R G G G O O O B B B;
                R R R G G G O O O B B B;
                W W W;
                W W W;
                W W W;
            ]
        );
    }

    #[test]
    fn test_ops() {
        use Operation::*;
        assert_ok!(parse_ops("U"), Ops::new(vec![Up(true)]));
        assert_ok!(
            parse_ops("U2F' x FU'U'"),
            Ops::new(vec![
                Up(true),
                Up(true),
                Front(false),
                X(true),
                Front(true),
                Up(false),
                Up(false),
            ])
        );
        assert_ok!(
            parse_ops("Lw Rw2 Lw' d2u2 u'"),
            Ops::new(vec![
                LeftDouble(true),
                RightDouble(true),
                RightDouble(true),
                LeftDouble(false),
                DownDouble(true),
                DownDouble(true),
                UpDouble(true),
                UpDouble(true),
                UpDouble(false),
            ])
        );
    }

    #[test]
    fn test_color() {
        use Color::*;
        assert_ok!(parse_color("Y"), Yellow);
        assert_ok!(parse_color("W"), White);
        assert_ok!(parse_color("."), Other);
        assert_ok!(parse_color("*"), Wildcard);
    }

    #[test]
    fn test_commentable_spaces() {
        assert_eq!(commentable_spaces("# comment"), Ok(("", ())));
        assert_eq!(commentable_spaces("    # a\n # b\n"), Ok(("", ())));
    }
}
