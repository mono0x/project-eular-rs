use std::collections::HashMap;

use num_bigint::BigUint;

// https://projecteuler.net/problem=25
#[allow(dead_code)]
fn problem_0025() -> u64 {
    struct State {
        memo: HashMap<u64, BigUint>,
    }

    impl State {
        fn fib(&mut self, n: u64) -> BigUint {
            if let Some(f) = self.memo.get(&n) {
                return f.clone();
            }
            let f = if n <= 2 {
                BigUint::from(1u64)
            } else {
                self.fib(n - 1) + self.fib(n - 2)
            };
            self.memo.insert(n, f.clone());
            f
        }
    }

    let mut state = State {
        memo: HashMap::new(),
    };

    let limit = BigUint::from(10u64).pow(999);

    for i in 1.. {
        let f = state.fib(i);
        if f >= limit {
            return i;
        }
    }
    panic!("must not happen")
}

#[test]
fn test_problem_0025() {
    assert_eq!(problem_0025(), 4782);
}
