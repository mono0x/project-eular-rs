// https://projecteuler.net/problem=26
#[allow(dead_code)]
fn problem_0026() -> u32 {
    let mut max = 0;
    let mut max_len = 0;

    for i in 2..1000 {
        let len = recurring_cycle_len(i);
        if len > max_len {
            max_len = len;
            max = i;
        }
    }
    max
}

fn recurring_cycle_len(d: u32) -> u32 {
    let mut digits: Vec<u32> = Vec::new();
    let mut reminders: Vec<u32> = Vec::new();
    let mut reminder = 1;
    while reminder > 0 {
        if let Some(pos) = reminders.iter().position(|&x| x == reminder) {
            return (digits.len() - pos) as u32;
        }
        reminders.push(reminder);

        digits.push(reminder * 10 / d);
        reminder = (reminder * 10) % d;
    }
    0 // no recurring cycle
}

#[test]
fn test_problem_0026() {
    assert_eq!(problem_0026(), 983);
}
