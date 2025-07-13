use core::panic;
use std::env::args;

use vietta_solver::solver::{solve, RootAmount};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 || args.len() > 3 { panic!("Please enter p and q as arguments\n(from a quadratic expression like x^2+px+q)"); }
    let (mut p, mut q): (i32, i32) = (0, 0);
    let mut iteration: u8 = 0;
    for arg in &args[1..=2] {
        iteration += 1;
        match arg.parse::<i32>() {
            Ok(num) => { 
                if iteration == 1 { p = num; } else {
                    q = num; }
             }
            Err(e) => {
                panic!("Could not parse argument Num.{}: {}\nAre you sure this is a valid integer?\n\n{}", iteration, arg, e);
            }
        }
    }
    if p == 0 { panic!("p cannot be zero"); }

    let result = solve(p, q);
    match result.roots_found() {
        RootAmount::None => {
            println!("Could not solve for roots using Vietta's Theorem!\nYour equation most likely has zero roots");
        }
        RootAmount::One => {
            println!("We found only one root!\nx1\t=\t{}", result.x1.unwrap());
        }
        RootAmount::Two => {
            println!("Solved!\nx1\t=\t{}\nx2\t=\t{}", result.x1.unwrap(), result.x2.unwrap());
        }
    }
}