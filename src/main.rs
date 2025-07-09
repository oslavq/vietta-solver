use std::env::args;

use vietta_solver::solver::solve;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 { panic!("Please enter p and q as arguments (from x^2+px+q)"); }
    let result = solve(args[1].parse().unwrap(), args[2].parse().unwrap());
    if result == [0, 0] { panic!("Could not solve for roots using the Vietta's theorem! Pass p and q as arguments from x^2+px+q"); }
    println!("Here is your result:\nX1\t=\t{}\nX2\t=\t{}", result[0], result[1]);
}