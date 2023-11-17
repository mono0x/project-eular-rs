// https://projecteuler.net/problem=18
#[allow(dead_code)]
fn problem_0018(triangle: &Vec<Vec<u64>>) -> u64 {
    calculate(triangle, 0, 0, 0)
}

fn calculate(triangle: &Vec<Vec<u64>>, x: usize, y: usize, sum: u64) -> u64 {
    if y == triangle.len() - 1 {
        return sum + triangle[y][x];
    }
    let mut max = 0;
    for i in 0..2 {
        max = std::cmp::max(max, calculate(triangle, x + i, y + 1, sum) + triangle[y][x])
    }
    max
}

#[rustfmt::skip]
#[test]
fn test_problem_0018() {
    assert_eq!(
        problem_0018(
            &vec![
                vec![3],
                vec![7, 4],
                vec![2, 4, 6],
                vec![8, 5, 9, 3]
            ]
        ),
        23
    );

    assert_eq!(
        problem_0018(
            &vec![
                vec![75],
                vec![95, 64],
                vec![17, 47, 82],
                vec![18, 35, 87, 10],
                vec![20,  4, 82, 47, 65],
                vec![19,  1, 23, 75,  3, 34],
                vec![88,  2, 77, 73,  7, 63, 67],
                vec![99, 65,  4, 28,  6, 16, 70, 92],
                vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
                vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
                vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
                vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
                vec![63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
                vec![ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60, 04, 23],
            ]
        ),
        1074
    )
}
