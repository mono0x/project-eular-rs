struct Month {
    year: u32,
    month: u32,
    weekday: u32, // Sunday = 0
}

// https://projecteuler.net/problem=19
#[allow(dead_code)]
fn problem_0019() -> u64 {
    let mut month = Month {
        year: 1900,
        month: 1,
        weekday: 1,
    };
    let mut sundays = 0;
    while month.year <= 2000 {
        if month.year >= 1901 && month.weekday == 0 {
            sundays += 1;
        }
        month = next(month);
    }
    sundays
}

fn next(month: Month) -> Month {
    let days = match month.month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (month.year % 4 == 0 && month.year % 100 != 0) || month.year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 31,
    };

    let last_month = month.month == 12;

    Month {
        year: if last_month {
            month.year + 1
        } else {
            month.year
        },
        month: if last_month { 1 } else { month.month + 1 },
        weekday: (month.weekday + days) % 7,
    }
}

#[test]
fn test_problem_0019() {
    assert_eq!(problem_0019(), 171);
}
