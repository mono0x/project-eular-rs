// https://projecteuler.net/problem=28
#[allow(dead_code)]
fn problem_0028(n: usize) -> u64 {
    let mut table = vec![vec![0u64; n]; n];

    let mut c = 1;
    let mut l = 0;
    let mut x = n / 2;
    let mut y = n / 2;
    loop {
        l += 1;

        let mut done = false;

        // right
        for _ in 0..l {
            table[y][x] = c;
            if x >= n - 1 {
                done = true;
                break;
            }
            x += 1;
            c += 1;
        }
        if done {
            break;
        }

        // down
        for _ in 0..l {
            table[y][x] = c;
            if y >= n - 1 {
                done = true;
                break;
            }
            y += 1;
            c += 1;
        }
        if done {
            break;
        }

        l += 1;

        // left
        for _ in 0..l {
            table[y][x] = c;
            if x == 0 {
                done = true;
                break;
            }
            x -= 1;
            c += 1;
        }
        if done {
            break;
        }

        // up
        for _ in 0..l {
            table[y][x] = c;
            if y == 0 {
                done = true;
                break;
            }
            y -= 1;
            c += 1;
        }
        if done {
            break;
        }
    }

    let mut result = 0;
    for i in 0..n {
        result += table[i][i];
        result += table[i][n - i - 1];
    }
    result -= table[n / 2][n / 2];

    result
}

#[test]
fn test_problem_0028() {
    assert_eq!(problem_0028(5), 101);
    assert_eq!(problem_0028(1001), 669171001);
}
