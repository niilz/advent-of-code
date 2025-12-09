fn main() {
    let inputs = inputs::fetch_input(1);

    let mut current = 50;

    let mut zeros = 0;
    for clicks in &inputs[..10] {
        dbg!(clicks);
        current = turn(current, clicks);
        if current == 0 {
            zeros += 1;
        }
    }
    dbg!(zeros);
}

fn turn(current: usize, turn: impl AsRef<str>) -> usize {
    let sign = &turn.as_ref()[0..1];
    let clicks = &turn.as_ref()[1..];
    let clicks = clicks.parse::<isize>().expect("parse click");
    let sign = if sign == "R" { 1 } else { -1isize };
    let num = current as isize + clicks * sign;
    dbg!(current, sign, clicks, num);
    let res = if num < 0 {
        // num is negative
        (100 + num) as usize
    } else if num > 99 {
        (num - 100) as usize
    } else {
        num as usize
    };
    dbg!(res);
    res
}
