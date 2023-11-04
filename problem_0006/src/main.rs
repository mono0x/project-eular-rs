fn main() {
    println!("{}", problem_0006());
}

// https://projecteuler.net/problem=6
fn problem_0006() -> u32 {
    let mut sum: u32 = 0;
    let mut sum_of_squares: u32 = 0;
    for i in 1..=100 {
        sum += i;
        sum_of_squares += i * i;
    }
    sum * sum - sum_of_squares
}

#[test]
fn test_problem_0006() {
    assert_eq!(problem_0006(), 25164150);
}
