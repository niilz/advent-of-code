use std::collections::VecDeque;

fn main() {
    let input = inputs::fetch_input(1);

    let mut wheel: VecDeque<usize> = (0..100).collect();
    wheel.rotate_left(50);

    let (_, zeros) =
        input[0..10]
            .iter()
            .map(to_number)
            .fold((50usize, 0), |(current, mut zeros), clicks| {
                println!("now: {current} turning {clicks}, zeros: {zeros}");
                dbg!(current, zeros, clicks);
                let cur = turn(current, clicks);
                if cur == 0 {
                    zeros += 1;
                }
                println!("after: {cur}, zeros: {zeros}");
                (cur, zeros)
            });
    dbg!(zeros);
}

fn to_number(turn: impl AsRef<str>) -> isize {
    let sign = &turn.as_ref()[0..1];
    dbg!(sign, turn.as_ref());
    let clicks = &turn.as_ref()[1..];
    let sign = if sign == "R" { 1 } else { -1 };
    dbg!(sign, clicks);
    clicks.parse::<isize>().expect("parse click") * sign
}

fn turn(current: usize, clicks: isize) -> usize {
    (current + clicks) % 99
}
