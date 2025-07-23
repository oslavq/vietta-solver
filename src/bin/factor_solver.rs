use vietta_solver::factors_find::find_factors;
use std::{env::{args, Args}, result};

fn main() {
    let args: Args = args();
    if args.len() > 2 || args.len() < 2 { panic!("Please enter the number factors of which you want to find as the only argyment") };
    let num: i32 = args.last().unwrap().parse()
        .expect("Could not parse the given argument, are you sure this is a valid integer?");

    let result = find_factors(num);
    if result.len() >= 1 {
        println!("Your number has {} factors:\n", result.len() * 2);
        for p in result {
            println!("{}\t*\t{}", p[0], p[1]);
        }
    } else {
        println!("Could not find factors for your number!")
    }
}