use crate::factors_find::find_factors;

// 1.   x1*x2=q  |   x1=q/x2
// 2.   x1+x2=-p |   x1=-p-x2
pub fn solve(p: i32, q: i32) -> [i32; 2] {
    let pairs_1: Vec<[i32; 2]> = find_factors(q); // pairs of numbers that satisfy the first boundary
    let (mut r1, mut r2): (i32, i32) = (0, 0);
    for pair in pairs_1 { // go over first set of pairs
        for enc_p in encode(pair) { // go over each encoded pair
            if enc_p[0] + enc_p[1] == -p { [r1, r2] = enc_p;};}};
    if r1 > r2 { [r2, r1] } else { [r1, r2] } // return roots in incremental order, so the smallest one always comes first
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