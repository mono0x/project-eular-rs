// https://projecteuler.net/problem=31
#[allow(dead_code)]
fn problem_0031(max: usize) -> u64 {
    static VALUES: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

    let mut dp = vec![0u64; max + 1];
    dp[0] = 1;
    for v in VALUES {
        for i in v..=max {
            dp[i] += dp[i - v];
        }
    }
    dp[max]
}

#[test]
fn test_problem_0031() {
    assert_eq!(problem_0031(200), 73682);
}
