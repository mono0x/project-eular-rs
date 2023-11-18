use num_bigint::BigUint;

// https://projecteuler.net/problem=20
#[allow(dead_code)]
fn problem_0020(input: u64) -> u64 {
    let mut n = BigUint::from(1u64);
    for i in 2..=input {
        n *= i;
    }
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

#[test]
fn test_problem_0020() {
    assert_eq!(problem_0020(10), 27);
    assert_eq!(problem_0020(100), 648);
}
