// https://projecteuler.net/problem=23
#[allow(dead_code)]
fn problem_0023() -> u64 {
    let mut abundant_numbers: Vec<u64> = Vec::new();
    for i in 1..=28123 {
        if factors(i).iter().sum::<u64>() > i {
            abundant_numbers.push(i);
        }
    }

    let mut abundant_sums = vec![false; 28123 + 1];
    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            let sum = abundant_numbers[i] + abundant_numbers[j];
            if sum <= 28123 {
                abundant_sums[sum as usize] = true;
            } else {
                break;
            }
        }
    }

    abundant_sums
        .iter()
        .enumerate()
        .filter(|(_, &b)| !b)
        .map(|(i, _)| i as u64)
        .sum()
}

fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            let j = n / i;
            if i != j && j != n {
                result.push(i);
                result.push(j);
            } else {
                result.push(i);
            }
        }
    }
    result
}

#[test]
fn test_problem_0023() {
    assert_eq!(problem_0023(), 4179871);
}
