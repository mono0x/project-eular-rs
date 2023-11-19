// https://projecteuler.net/problem=30
#[allow(dead_code)]
fn problem_0030(n: u32) -> u32 {
    let mut sum = 0;
    for digits in 1.. {
        // 9**5 * digits is the maximum number of the sum of the fifth power of digits
        // 10**(digits-1) is the minimum number of the `digits`
        // if the max of fifth power of digits is less than the min of `digits`, then we can stop.
        if 9u32.pow(n) * digits < 10u32.pow(digits - 1) {
            break;
        }

        for i in 10u32.pow(digits - 1)..10u32.pow(digits) {
            let mut j = i;
            let mut s = 0;
            for _ in 0..digits {
                let d = j % 10;
                s += d.pow(n);
                j /= 10;
            }
            if s == i {
                sum += s;
            }
        }
    }

    sum - 1 // 1 is excluded from the sum
}

#[test]
fn test_problem_0030() {
    assert_eq!(problem_0030(4), 19316);
    assert_eq!(problem_0030(5), 443839);
}
