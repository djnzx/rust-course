///  https://www.hackerrank.com/challenges/counting-valleys/problem
fn countingValleys1(steps: i32, path: &str) -> i32 {
    struct State {
        level: i32,
        count: i32,
    }

    impl State {
        fn new() -> Self {
            Self { level: 0, count: 0 }
        }
    }

    path.chars()
        .fold(State::new(), |st, c| {
            let curr_level = match c {
                'U' => st.level + 1,
                'D' => st.level - 1,
                _ => panic!("wrong char"),
            };
            let delta_count = match curr_level {
                0 if st.level < 0 => 1,
                _ => 0,
            };
            State {
                level: curr_level,
                count: st.count + delta_count,
            }
        })
        .count
}

fn countingValleys2(steps: i32, path: &str) -> i32 {
    let mut prev_level = 0;
    let mut count = 0;
    for c in path.chars() {
        let curr_level = match c {
            'U' => prev_level + 1,
            'D' => prev_level - 1,
            _ => prev_level,
        };
        let delta = match curr_level {
            0 if prev_level < 0 => 1,
            _ => 0,
        };
        prev_level = curr_level;
        count += delta;
    }
    count
}
