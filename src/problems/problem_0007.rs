// https://projecteuler.net/problem=7
#[allow(dead_code)]
pub fn problem_0007() -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    for i in 2.. {
        if primes.iter().all(|&x| i % x != 0) {
            if primes.len() + 1 == 10001 {
                return i;
            }
            primes.push(i);
        }
    }
    panic!("Not enough primes")
}

#[test]
fn test_problem_0007() {
    assert_eq!(problem_0007(), 104743);
}
