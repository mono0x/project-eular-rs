// https://projecteuler.net/problem=31
#[allow(dead_code)]
fn problem_0031(max: u32) -> u32 {
    static VALUES: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

    fn search(index: usize, max: u32, sum: u32) -> u32 {
        if sum == max {
            return 1;
        }
        if index == 8 {
            return 0;
        }

        if index == 3 && sum % 10 != 0 {
            return 0;
        }
        if index == 6 && sum % 100 != 0 {
            return 0;
        }

        let mut result = 0;
        for i in 0..=max {
            let next = sum + i * VALUES[index];
            if next > max {
                break;
            }
            result += search(index + 1, max, next);
        }
        result
    }

    search(0, max, 0)
}

#[test]
fn test_problem_0031() {
    assert_eq!(problem_0031(200), 73682);
}
