// https://projecteuler.net/problem=27
#[allow(dead_code)]
fn problem_0027() -> i32 {
    let mut primes = [true; 100_000];
    primes[0] = false;
    primes[1] = false;
    for i in 2..primes.len() {
        if primes[i] {
            for j in (i + i..primes.len()).step_by(i) {
                primes[j] = false;
            }
        }
    }

    let mut max_len = 0;
    let mut max = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            for n in 0..80 {
                let x = n * n + a * n + b;
                if x < 0 || !primes[x as usize] {
                    if n > max_len {
                        max_len = n;
                        max = a * b;
                    }
                    break;
                }
            }
        }
    }
    max
}

#[test]
fn test_problem_0027() {
    assert_eq!(problem_0027(), -59231);
}
