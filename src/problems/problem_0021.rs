// https://projecteuler.net/problem=21
#[allow(dead_code)]
fn problem_0021() -> u64 {
    let mut ds = vec![0u64; 10000];
    for i in 1..10000 {
        ds[i] = d(i as u64);
    }

    let mut result = 0u64;
    for i in 1..10000 {
        let j = ds[i] as usize;
        if j < 10000 && i != j && ds[j] == i as u64 {
            result += i as u64;
        }
    }
    result
}

fn d(n: u64) -> u64 {
    let mut result = 1;
    for (p, i) in prime_factorize(n) {
        result *= (0..=i).map(|_| p.pow(i as u32)).sum::<u64>();
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
fn test_problem_0021() {
    assert_eq!(problem_0021(), 0);
}
