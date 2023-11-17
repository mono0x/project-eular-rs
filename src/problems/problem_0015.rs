// https://projecteuler.net/problem=15
#[allow(dead_code)]
fn problem_0015(size: usize) -> u64 {
    let point_size = size + 1;
    let mut points = vec![vec![0u64; point_size + 2]; point_size + 2];
    points[1][1] = 1;
    for i in 1..=point_size {
        for j in 1..=point_size {
            if i == 1 && j == 1 {
                continue;
            }
            points[i][j] = points[i - 1][j] + points[i][j - 1];
        }
    }
    points[point_size][point_size]
}

#[test]
fn test_problem_0015() {
    assert_eq!(problem_0015(2), 6);
    assert_eq!(problem_0015(20), 137846528820);
}
