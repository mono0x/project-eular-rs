// https://projecteuler.net/problem=2
pub fn problem_0002() -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 2;
    let mut sum: u32 = b;
    while b < 4_000_000 {
        let next = a + b;
        if next % 2 == 0 {
            sum += next
        }
        a = b;
        b = next
    }
    sum
}

#[test]
fn test_problem_0002() {
    assert_eq!(problem_0002(), 4613732);
}
