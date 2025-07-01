use std::{collections::HashSet, env::args};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 { panic!("Please enter p and q as arguments (from x^2+px+q)"); }
    let result = solve(args[1].parse().unwrap(), args[2].parse().unwrap());
    if result == [0, 0] { panic!("Could not solve for roots using the Vietta's theorem! Pass p and q as arguments from x^2+px+q"); }
    println!("Here is your result:\nX1\t=\t{}\nX2\t=\t{}", result[0], result[1]);
}

// 1.   x1*x2=q  |   x1=q/x2
// 2.   x1+x2=-p |   x1=-p-x2
fn solve(p: i32, q: i32) -> [i32; 2] {
    let pairs_1: Vec<[i32; 2]> = find_factors(q); // pairs of numbers that satisfy the first boundary
    let (mut r1, mut r2): (i32, i32) = (0, 0);
    for pair in pairs_1 { // go over first set of pairs
        for enc_p in encode(pair) { // go over each encoded pair
            if enc_p[0] + enc_p[1] == -p { [r1, r2] = enc_p;};}};
    if r1 > r2 { [r2, r1] } else { [r1, r2] } // return roots in incremental order, so the smallest one always comes first
    }

fn find_factors(num: i32) -> Vec<[i32; 2]> {
    let mut vector: Vec<[i32; 2]> = vec![[1, num.abs()]];
    let mut used_nums: HashSet<i32> = HashSet::new(); // avoid repetition
    for i in 2..=((num as f32 / 2f32).ceil() as i32 + 1).abs() { // iterate over nums from 2 to the half of q
        if used_nums.contains(&i) || i == num.abs() { break } // if i is a number on position 2 from any previous entry
        if (num.abs() % i) == 0 {vector.push([i, num.abs() / i]); // if i turns out to be a factor if num
            used_nums.insert(num.abs() / i);}} // save second factor
    return vector
}

fn encode(pair: [i32; 2]) -> [[i32;2]; 4] {
       [[pair[0],   pair[1]],
        [-pair[0],  pair[1]],
        [pair[0],  -pair[1]],
        [-pair[0], -pair[1]]]
}

#[test]
fn test_factors() {
    let values: [(i32, Vec<[i32; 2]>); 5] =
           [(3, vec![[1, 3]]),
            (6, vec![[1, 6], [2, 3]]),
            (9, vec![[1, 9], [3, 3]]),
            (16, vec![[1, 16], [2, 8], [4, 4]]),
            (32, vec![[1, 32], [2, 16], [4, 8]]),];
    for tv in values {
        assert_eq!(find_factors(tv.0), tv.1);
        assert_eq!(find_factors(-tv.0), tv.1);}
}

#[test]
fn test_encoder() {
    let values: [([i32; 2], [[i32;2]; 4]); 3] =
        [   ([7, -12],  [[7, -12], [-7, -12], [7, 12], [-7, 12]]),
            ([10, 5],   [[10, 5], [-10, 5], [10, -5], [-10, -5]]),
            ([-3, 12],  [[-3, 12], [3, 12], [-3, -12], [3, -12]])];
    for value in values {
        assert_eq!(encode(value.0), value.1);}
}

#[test]
fn test_solver() {
    let values: [[[i32; 2] ;2] ;4] =
           [[[-28, 96], [4, 24]],
            [[-5, -6], [2, 3]],
            [[2, -120], [-12, 10]],
            [[31, -180], [-36, 5]]];
    for val in values {
        assert_eq!(solve(val[0][0], val[0][1]), val[1]);}
}