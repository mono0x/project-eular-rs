// https://projecteuler.net/problem=28
#[allow(dead_code)]
fn problem_0028(n: usize) -> usize {
    let mut result = 1;
    let mut i = 1;
    for step in (2..n).step_by(2) {
        /*
            for _ in 0..4 {
                i += step;
                result += i;
            }
        */
        result += 4 * i + (1 + 2 + 3 + 4) * step;
        i += step * 4;
    }
    result
}

#[test]
fn test_problem_0028() {
    assert_eq!(problem_0028(5), 101);
    assert_eq!(problem_0028(1001), 669171001);
}
