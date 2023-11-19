use std::collections::HashSet;

use num_bigint::BigUint;

// https://projecteuler.net/problem=29
#[allow(dead_code)]
fn problem_0029(max_a: u32, max_b: u32) -> usize {
    let mut terms = HashSet::new();
    for a in 2..=max_a {
        for b in 2..=max_b {
            terms.insert(BigUint::from(a).pow(b));
        }
    }
    terms.len()
}

#[test]
fn test_problem_0029() {
    assert_eq!(problem_0029(5, 5), 15);
    assert_eq!(problem_0029(100, 100), 9183);
}
