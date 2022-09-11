mod entities;
mod read;
mod solver;
mod util;

use crate::entities::*;
use log::{error, info, warn};
use read::{cat, read};
use serde_json::json;
use std::env;
use structopt::StructOpt;
use Operation::*;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, default_value = "9")]
    max_depth: usize,
    #[structopt(short, long, default_value = "1", help = "max num of algorithm")]
    num: usize,
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(short, long)]
    quiet: bool,

    #[structopt(short = "U")]
    up: bool,
    #[structopt(short = "D")]
    down: bool,
    #[structopt(short = "F")]
    front: bool,
    #[structopt(short = "B")]
    back: bool,
    #[structopt(short = "L")]
    left: bool,
    #[structopt(short = "R")]
    right: bool,

    #[structopt(short = "u", long = "Uw")]
    up_double: bool,
    #[structopt(short = "d", long = "Dw")]
    down_double: bool,
    #[structopt(short = "f", long = "Fw")]
    front_double: bool,
    #[structopt(short = "b", long = "Bw")]
    back_double: bool,
    #[structopt(short = "l", long = "Lw")]
    left_double: bool,
    #[structopt(short = "r", long = "Rw")]
    right_double: bool,

    #[structopt(short = "M", long)]
    middle: bool,
    #[structopt(short = "E", long)]
    equator: bool,
    #[structopt(short = "S", long)]
    standing: bool,

    #[structopt(short)]
    x: bool,
    #[structopt(short)]
    y: bool,
    #[structopt(short)]
    z: bool,

    #[structopt(
        long,
        help = "Complete solve by CFOP method, other options are all ignored"
    )]
    cfop: bool,

    #[structopt(
        long,
        help = "Complete solve by Roux method, other options are all ignored"
    )]
    roux: bool,
}

fn validation(c: &Cube, d: &Cube) -> Result<(), Color> {
    let count = c.count();
    let dount = d.count();
    let cwild = count[&Color::Wildcard];
    let dwild = dount[&Color::Wildcard];
    for col in vec![
        Color::Red,
        Color::Blue,
        Color::Yellow,
        Color::White,
        Color::Orange,
        Color::Green,
        Color::Other,
    ] {
        if count[&col] > dount[&col] + dwild {
            return Err(col);
        }
        if count[&col] + cwild < dount[&col] {
            return Err(col);
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    env::set_var("RUST_LOG", if !opt.quiet { "info" } else { "error" });
    env_logger::builder()
        .format_target(false)
        .format_indent(Some(0))
        .init();

    let mut allowed_ops = vec![];
    if opt.up {
        allowed_ops.push(Up(true));
        allowed_ops.push(Up(false));
    }
    if opt.down {
        allowed_ops.push(Down(true));
        allowed_ops.push(Down(false));
    }
    if opt.front {
        allowed_ops.push(Front(true));
        allowed_ops.push(Front(false));
    }
    if opt.back {
        allowed_ops.push(Back(true));
        allowed_ops.push(Back(false));
    }
    if opt.left {
        allowed_ops.push(Left(true));
        allowed_ops.push(Left(false));
    }
    if opt.right {
        allowed_ops.push(Right(true));
        allowed_ops.push(Right(false));
    }
    if opt.up_double {
        allowed_ops.push(UpDouble(true));
        allowed_ops.push(UpDouble(false));
    }
    if opt.down_double {
        allowed_ops.push(DownDouble(true));
        allowed_ops.push(DownDouble(false));
    }
    if opt.front_double {
        allowed_ops.push(FrontDouble(true));
        allowed_ops.push(FrontDouble(false));
    }
    if opt.back_double {
        allowed_ops.push(BackDouble(true));
        allowed_ops.push(BackDouble(false));
    }
    if opt.left_double {
        allowed_ops.push(LeftDouble(true));
        allowed_ops.push(LeftDouble(false));
    }
    if opt.right_double {
        allowed_ops.push(RightDouble(true));
        allowed_ops.push(RightDouble(false));
    }
    if opt.middle {
        allowed_ops.push(Middle(true));
        allowed_ops.push(Middle(false));
    }
    if opt.equator {
        allowed_ops.push(Equator(true));
        allowed_ops.push(Equator(false));
    }
    if opt.standing {
        allowed_ops.push(Standing(true));
        allowed_ops.push(Standing(false));
    }
    if opt.x {
        allowed_ops.push(X(true));
        allowed_ops.push(X(false));
    }
    if opt.y {
        allowed_ops.push(Y(true));
        allowed_ops.push(Y(false));
    }
    if opt.z {
        allowed_ops.push(Z(true));
        allowed_ops.push(Z(false));
    }

    let (cube, goal) = read(cat().as_str());
    info!("Init\n{}", &cube);
    info!("Goal\n{}", &goal);
    if let Err(col) = validation(&cube, &goal) {
        error!("Validation Failed. Check number of color:{}.", col);
        return;
    }

    if opt.cfop {
        if let Some(alg) = solver::cfop(&cube, opt.verbose) {
            println!(
                "{}",
                json!({
                    "ok": true,
                    "solution": {
                        "algorithm": format!("{}", alg),
                        "length": alg.len(),
                    }
                })
            );
        } else {
            info!("No Solution");
            println!("{}", json!({ "ok": false, "solution": {} }));
        }
        return;
    }

    if opt.roux {
        if let Some(alg) = solver::roux(&cube, opt.verbose) {
            println!(
                "{}",
                json!({
                    "ok": true,
                    "solution": {
                        "algorithm": format!("{}", alg),
                        "length": alg.len(),
                    }
                })
            );
        } else {
            info!("No Solution");
            println!("{}", json!({ "ok": false, "solution": {} }));
        }
        return;
    }

    if allowed_ops.is_empty() {
        error!("No Operations specified");
        return;
    }

    if opt.max_depth >= 10 {
        warn!("Too large max_depth: {}", opt.max_depth);
    }

    let algorithms = solver::search(
        &cube,
        &goal,
        allowed_ops,
        opt.max_depth,
        opt.num,
        opt.verbose,
    );
    if !algorithms.is_empty() {
        let mut solutions = vec![];
        for ops in algorithms.iter() {
            solutions.push(json!({
                        "algorithm": format!("{}", ops),
                        "length": ops.len(),
            }));
        }
        println!("{}", json!({ "ok": true, "solutions": solutions, }));
    } else {
        info!("No Solution");
        println!("{}", json!({ "ok": false, "solutions": [], }));
    }
}
