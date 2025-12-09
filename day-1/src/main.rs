use std::collections::VecDeque;

fn main() {
    let inputs = inputs::fetch_input(1);

    let mut wheel: VecDeque<usize> = (0..100).collect();
    wheel.rotate_left(50);

    let mut zeros = 0;
    for input in &inputs[..105] {
        let (clicks, rotation) = action(input);
        rotation(&mut wheel, clicks);
        let first = wheel.get(0).expect("read at idx 0");
        dbg!(first);
        if first == &0 {
            zeros += 1;
        }
        dbg!(input);
    }
    dbg!(zeros);
}

fn action(turn: impl AsRef<str>) -> (usize, fn(&mut VecDeque<usize>, usize)) {
    let sign = &turn.as_ref()[0..1];
    let clicks = &turn.as_ref()[1..];
    let clicks = clicks.parse::<usize>().expect("parse click");
    if sign == "R" {
        (clicks, |mut vd, clicks| {
            dbg!(vd.len(), clicks);
            VecDeque::rotate_left(&mut vd, clicks)
        })
    } else {
        (clicks, |mut vd, clicks| {
            VecDeque::rotate_right(&mut vd, clicks)
        })
    }
}
