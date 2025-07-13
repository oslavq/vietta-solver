use std::env::args;

use vietta_solver::solver::{solve, RootAmount};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 { panic!("Please enter p and q as arguments (from x^2+px+q)"); }
    let result = solve(args[1].parse().unwrap(), args[2].parse().unwrap());
    match result.roots_found() {
        RootAmount::None => {
            println!("Could not solve for roots using Vietta's Theorem!");
        }
        RootAmount::One => {
            println!("We found only one root!\nx1\t=\t{}", result.x1.unwrap());
        }
        RootAmount::Two => {
            println!("Solved!\nx1\t=\t{}\nx2\t=\t{}", result.x1.unwrap(), result.x2.unwrap());
        }
    }
}