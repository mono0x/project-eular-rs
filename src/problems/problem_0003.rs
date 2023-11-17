// https://projecteuler.net/problem=3
#[allow(dead_code)]
fn problem_0003(input: u64) -> u64 {
    let mut n = input;
    for i in 2.. {
        if n % i == 0 {
            n /= i;
            if n == 1 {
                return i;
            }
        }
    }
    panic!("must not happen")
}

#[test]
fn test_problem_0003() {
    assert_eq!(problem_0003(13195), 29);
    assert_eq!(problem_0003(600851475143), 6857);
}
