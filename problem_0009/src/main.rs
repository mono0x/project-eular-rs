fn main() {
    println!("{}", problem_0009());
}

// https://projecteuler.net/problem=9
fn problem_0009() -> i64 {
    for a in 1..=333 {
        for b in a..=500 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    panic!("No solution found")
}

#[test]
fn test_problem_0009() {
    assert_eq!(problem_0009(), 31875000);
}
