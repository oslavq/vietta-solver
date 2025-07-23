use crate::solver::{Solution, RootAmount};

pub fn validate(p: i32, q: i32, sol: Solution) -> bool {
    match sol.roots_found() {
        RootAmount::None => { false }
        RootAmount::One => {
            calc_with_formula(p, q, sol.x1.unwrap()) == 0
        }
        RootAmount::Two => {
            calc_with_formula(p, q, sol.x1.unwrap()) == 0
            && calc_with_formula(p, q, sol.x2.unwrap()) == 0
        }
    }
}

fn calc_with_formula(p: i32, q: i32, x: i32) -> i32 {
    (x * x) + (p * x) + (q)
}


#[test]
fn test_validation() {
    let values: [(i32, i32, Solution, bool); 2] = [
        (2, -120, Solution::new(-12, 10), true),
        (4, 17, Solution::new(5, -3), false),
    ];

    for v in values {
        assert_eq!(validate(v.0, v.1, v.2), v.3);
    }
}