use num_bigint::BigUint;

// https://projecteuler.net/problem=25
#[allow(dead_code)]
fn problem_0025() -> u64 {
    let mut a = BigUint::from(1u64);
    let mut b = BigUint::from(1u64);

    let limit = BigUint::from(10u64).pow(999);

    for i in 3.. {
        let c = &a + &b;
        if c >= limit {
            return i;
        }
        a = b;
        b = c;
    }
    panic!("must not happen")
}

#[test]
fn test_problem_0025() {
    assert_eq!(problem_0025(), 4782);
}
