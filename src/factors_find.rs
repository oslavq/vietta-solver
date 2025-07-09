use std::collections::HashSet;

pub fn find_factors(num: i32) -> Vec<[i32; 2]> {
    let mut vector: Vec<[i32; 2]> = vec![[1, num.abs()]];
    let mut used_nums: HashSet<i32> = HashSet::new(); // avoid repetition
    for i in 2..=((num as f32 / 2f32).ceil() as i32 + 1).abs() { // iterate over nums from 2 to the half of q
        if used_nums.contains(&i) || i == num.abs() { break } // if i is a number on position 2 from any previous entry
        if (num.abs() % i) == 0 {vector.push([i, num.abs() / i]); // if i turns out to be a factor if num
            used_nums.insert(num.abs() / i);}} // save second factor
    return vector
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