// https://projecteuler.net/problem=12
#[allow(dead_code)]
fn problem_0012(divisors: u64) -> u64 {
    for i in 1.. {
        let n = i * (i + 1) / 2;
        if count_divisors(n) > divisors {
            return n;
        }
    }
    panic!("must not happen")
}

fn count_divisors(n: u64) -> u64 {
    let mut result = 1;
    for (_, i) in prime_factorize(n) {
        result *= i + 1;
    }
    result
}

fn prime_factorize(n: u64) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();
    let mut x = 2;
    let mut m = n;
    while x <= m {
        for i in 0.. {
            if m % x != 0 {
                if i > 0 {
                    result.push((x, i));
                }
                break;
            }
            m /= x;
        }
        // x = 2, 3, 5, 7, 9, ...
        // even numbers except 2 are not prime
        x += if x > 2 { 2 } else { 1 };
    }
    result
}

#[test]
fn test_problem_0012() {
    assert_eq!(problem_0012(5), 28);
    assert_eq!(problem_0012(500), 76576500);
}
