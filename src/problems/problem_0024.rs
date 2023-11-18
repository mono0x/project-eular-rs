// https://projecteuler.net/problem=24
#[allow(dead_code)]
fn problem_0024() -> u64 {
    struct State {
        buffer: [i32; 10],
        used: [bool; 10],
        result: u64,
        order: usize,
        done: bool,
    }

    impl State {
        fn search(&mut self, pos: usize) {
            if pos == 10 {
                self.order += 1;
                if self.order == 1_000_000 {
                    let mut r = 0;
                    for d in self.buffer.iter() {
                        r *= 10;
                        r += *d as u64;
                    }
                    self.result = r;
                    self.done = true;
                }
                return;
            }
            for n in 0..=9 {
                if self.used[n as usize] {
                    continue;
                }
                self.buffer[pos] = n;
                self.used[n as usize] = true;
                self.search(pos + 1);
                self.used[n as usize] = false;
                if self.done {
                    break;
                }
            }
        }
    }

    let mut state = State {
        buffer: [-1; 10],
        used: [false; 10],
        result: 0,
        order: 0,
        done: false,
    };

    state.search(0);
    state.result
}

#[test]
fn test_problem_0024() {
    assert_eq!(problem_0024(), 2783915460);
}
