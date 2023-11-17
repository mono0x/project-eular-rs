// https://projecteuler.net/problem=10
#[allow(dead_code)]
pub fn problem_0010(input: usize) -> u64 {
    // new vector with size and filled by initial value
    let mut primes: Vec<bool> = vec![true; input + 1];
    primes[1] = false;
    for i in 1..=input {
        if primes[i] {
            for j in (i + i..=input).step_by(i) {
                primes[j] = false;
            }
        }
    }
    (1..=input).filter(|&x| primes[x]).map(|x| x as u64).sum()
}

#[test]
fn test_problem_0010() {
    assert_eq!(problem_0010(10), 17);
    assert_eq!(problem_0010(2_000_000), 142913828922);
}
