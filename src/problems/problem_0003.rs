// https://projecteuler.net/problem=3
pub fn problem_0003(input: u64) -> u64 {
    let mut primes: Vec<u64> = Vec::new();
    for i in 2..((input as f64).sqrt()) as u64 {
        if primes.iter().all(|&x| i % x != 0) {
            primes.push(i)
        }
    }
    primes
        .iter()
        .rev()
        .find(|&x| input % x == 0)
        .unwrap()
        .clone()
}

#[test]
fn test_problem_0003() {
    assert_eq!(problem_0003(13195), 29);
    assert_eq!(problem_0003(600851475143), 6857);
}
