// https://projecteuler.net/problem=14
#[allow(dead_code)]
fn problem_0014() -> u64 {
    let mut max = 0;
    let mut max_i = 0;
    for i in 2..1_000_000u64 {
        let mut n = i;
        for count in 1.. {
            if n == 1 {
                if count > max {
                    max = count;
                    max_i = i;
                }
                break;
            }
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
    }
    max_i
}

#[test]
fn test_problem_0014() {
    assert_eq!(problem_0014(), 837799);
}
