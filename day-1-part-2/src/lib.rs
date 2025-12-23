pub fn turn(current: usize, turn: impl AsRef<str>) -> (usize, usize) {
    let started_on_zero = if current == 0 { 0 } else { 1 };
    let sign = &turn.as_ref()[0..1];
    let clicks = &turn.as_ref()[1..];
    let clicks = clicks.parse::<usize>().expect("parse click");
    // example: if R256 the turn is two times to the same position plus 56
    let full_turns = clicks / 100;
    let clicks_normalized = clicks % 100;
    let sign = if sign == "R" { 1 } else { -1isize };
    let num = current as isize + clicks_normalized as isize * sign;
    //println!("current: {current}, sign: {sign}, clicks: {clicks}, num: {num})");
    let (res, zeros) = if num < 0 {
        // num is negative
        //println!("100 + {num} = {}", 100 + num);
        let landed = (100 + num) as usize;
        (landed, full_turns + started_on_zero)
    } else if num > 99 {
        //println!("{num} - 100 = {}", num - 100);
        let landed = (num - 100) as usize;
        (landed, full_turns + started_on_zero)
    } else if num == 0 {
        (num as usize, full_turns + started_on_zero)
    } else {
        (num as usize, full_turns)
    };
    //dbg!(res);
    (res, zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_turns() {
        // The dial starts by pointing at 50.
        let start = 50;
        // The dial is rotated L68 to point at 82.
        let (actual, zeros) = turn(start, "L68");
        assert_eq!((actual, zeros), (82, 1));
        // The dial is rotated L30 to point at 52.
        let (actual, zeros) = turn(actual, "L30");
        assert_eq!((actual, zeros), (52, 0));
        // The dial is rotated R48 to point at 0.
        let (actual, zeros) = turn(actual, "R48");
        assert_eq!((actual, zeros), (0, 1));
        // The dial is rotated L5 to point at 95.
        let (actual, zeros) = turn(actual, "L5");
        assert_eq!((actual, zeros), (95, 0));
        // The dial is rotated R60 to point at 55.
        let (actual, zeros) = turn(actual, "R60");
        assert_eq!((actual, zeros), (55, 1));
        // The dial is rotated L55 to point at 0.
        let (actual, zeros) = turn(actual, "L55");
        assert_eq!((actual, zeros), (0, 1));
        // The dial is rotated L1 to point at 99.
        let (actual, zeros) = turn(actual, "L1");
        assert_eq!((actual, zeros), (99, 0));
        // The dial is rotated L99 to point at 0.
        let (actual, zeros) = turn(actual, "L99");
        assert_eq!((actual, zeros), (0, 1));
        // The dial is rotated R14 to point at 14.
        let (actual, zeros) = turn(actual, "R14");
        assert_eq!((actual, zeros), (14, 0));
        // The dial is rotated L82 to point at 32.
        let (actual, zeros) = turn(actual, "L82");
        assert_eq!((actual, zeros), (32, 1));
    }
}
