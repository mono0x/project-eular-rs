// https://projecteuler.net/problem=5
pub fn problem_0005() -> u32 {
    for i in (20..).step_by(20) {
        if (1..=20).all(|x| i % x == 0) {
            return i;
        }
    }
    panic!("No solution found");
}

#[test]
fn test_problem_0005() {
    assert_eq!(problem_0005(), 232792560);
}
