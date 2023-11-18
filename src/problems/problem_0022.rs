const NAMES: &str = include_str!("../../resources/problem_0022/0022_names.txt");

// https://projecteuler.net/problem=22
#[allow(dead_code)]
fn problem_0022() -> u64 {
    let mut names = NAMES
        .split(",")
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<_>>();
    names.sort();

    let mut result = 0;
    for (i, name) in names.iter().enumerate() {
        let score = name.chars().map(|c| c as u64 - 'A' as u64 + 1).sum::<u64>() * (i as u64 + 1);
        result += score;
    }
    result
}

#[test]
fn test_problem_0022() {
    assert_eq!(problem_0022(), 871198282);
}
