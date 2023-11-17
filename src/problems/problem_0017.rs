const LENGTH_OF_NUMBERS: [u64; 20] = [
    0, // 0
    3, // 1
    3, // 2
    5, // 3
    4, // 4
    4, // 5
    3, // 6
    5, // 7
    5, // 8
    4, // 9
    3, // 10
    6, // 11
    6, // 12
    8, // 13
    8, // 14
    7, // 15
    7, // 16
    9, // 17
    8, // 18
    8, // 19
];

const LENGTH_OF_TENS: [u64; 10] = [
    0, // 0
    0, // 10
    6, // 20
    6, // 30
    5, // 40
    5, // 50
    5, // 60
    7, // 70
    6, // 80
    6, // 90
];

// https://projecteuler.net/problem=17
#[allow(dead_code)]
fn problem_0017(input: usize) -> u64 {
    (1..=input).map(|i| calculate_length(i)).sum()
}

fn calculate_length(i: usize) -> u64 {
    if i < 20 {
        LENGTH_OF_NUMBERS[i]
    } else if i < 100 {
        LENGTH_OF_TENS[i / 10] + calculate_length(i % 10)
    } else if i < 1000 {
        LENGTH_OF_NUMBERS[i / 100]
            + 7 // hundred
            + if i % 100 != 0 {
                3 + calculate_length(i % 100) // and + length of the rest
            } else {
                0
            }
    } else if i == 1000 {
        calculate_length(i / 1000) + 8 // one thousand
    } else {
        panic!("must not happen")
    }
}

#[test]
fn test_problem_0017() {
    assert_eq!(problem_0017(5), 19);
    assert_eq!(problem_0017(1000), 21124);
}
