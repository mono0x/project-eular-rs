use num_bigint::BigUint;

#[allow(dead_code)]
fn problem_0016(power: u64) -> u64 {
    let n = BigUint::from(2u64) << (power - 1);
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

#[test]
fn test_problem_0016() {
    assert_eq!(problem_0016(15), 26);
    assert_eq!(problem_0016(1000), 1366);
}
