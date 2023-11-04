fn main() {
    println!("{}", problem_0001(1000));
}

// https://projecteuler.net/problem=1
fn problem_0001(input: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 3..input {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    sum
}

#[test]
fn test_problem_0001() {
    assert_eq!(problem_0001(10), 23);
    assert_eq!(problem_0001(1000), 233168);
}
