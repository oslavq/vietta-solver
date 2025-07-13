use crate::factors_find::find_factors;

#[derive(PartialEq, Debug)]
pub struct Solution {
    pub x1: Option<i32>,
    pub x2: Option<i32>,
}

pub enum RootAmount {
    None,
    One,
    Two
}

impl Solution {
    fn new_blank() -> Solution { Self { x1: None, x2: None } }
    fn new(x1: i32, x2: i32) -> Self { Self { x1: Some(x1), x2: Some(x2) }}

    pub fn roots_found(&self) -> RootAmount {
        if self.x1 == None && self.x2 == None {
            RootAmount::None } else if self.x2 == None {
                RootAmount::One
            } else {
                RootAmount::Two }
    }
}

// 1.   x1*x2=q  |   x1=q/x2
// 2.   x1+x2=-p |   x1=-p-x2
pub fn solve(p: i32, q: i32) -> Solution {
    let pairs_1: Vec<[i32; 2]> = find_factors(q); // pairs of numbers that satisfy the first boundary
    let mut sol = Solution::new_blank();
    for pair in pairs_1 { // go over first set of pairs
        for enc_p in encode(pair) { // go over each encoded pair
            if enc_p[0] + enc_p[1] == -p { // found a valid pair
                if enc_p[0] == enc_p[1] { // if there is only one root
                    sol = Solution { x1: Some(enc_p[0]), x2: None };
                } else { // if there are two roots, sort the values
                  if enc_p[0] < enc_p[1] {
                    sol.x1 = Some(enc_p[0]); sol.x2 = Some(enc_p[1]);
                  } else {
                    sol.x1 = Some(enc_p[1]); sol.x2 = Some(enc_p[0]);
                  }
                }
            };}};
    return sol;
}

fn encode(pair: [i32; 2]) -> [[i32;2]; 4] {
       [[pair[0],   pair[1]],
        [-pair[0],  pair[1]],
        [pair[0],  -pair[1]],
        [-pair[0], -pair[1]]]
}

#[test]
fn test_encoder() {
    let values: [([i32; 2], [[i32;2]; 4]); 3] =
        [   ([7, -12],  [[7, -12], [-7, -12], [7, 12], [-7, 12]]),
            ([10, 5],   [[10, 5], [-10, 5], [10, -5], [-10, -5]]),
            ([-3, 12],  [[-3, 12], [3, 12], [-3, -12], [3, -12]])];
    for value in values {
        assert_eq!(encode(value.0), value.1);
}}

#[test]
fn test_solver() {
    let values: [([i32; 2], Solution) ;4] =
        [([-28, 96], Solution::new(4, 24)),
        ([-5, -6], Solution::new(2, 3)),
        ([2, -120], Solution::new(-12, 10)),
        ([31, -180], Solution::new(-36, 5))];
    for val in values {
        assert_eq!(solve(val.0[0], val.0[1]), val.1);
}}