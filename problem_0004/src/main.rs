fn main() {
    println!("{}", problem_0004());
}

// https://projecteuler.net/problem=4
fn problem_0004() -> u32 {
    let mut max: u32 = 0;
    for i in (100..=999).rev() {
        if i * 999 < max {
            break;
        }
        for j in (i..=999).rev() {
            if is_palindrome(i * j) {
                max = std::cmp::max(max, i * j);
                break;
            }
        }
    }
    max
}

fn is_palindrome(input: u32) -> bool {
    input.to_string() == input.to_string().chars().rev().collect::<String>()
}

#[test]
fn test_problem_0004() {
    assert_eq!(problem_0004(), 906609);
}
