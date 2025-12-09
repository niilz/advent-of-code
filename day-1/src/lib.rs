/*
    The dial starts by pointing at 50.
    The dial is rotated L68 to point at 82.
    The dial is rotated L30 to point at 52.
    The dial is rotated R48 to point at 0.
    The dial is rotated L5 to point at 95.
    The dial is rotated R60 to point at 55.
    The dial is rotated L55 to point at 0.
    The dial is rotated L1 to point at 99.
    The dial is rotated L99 to point at 0.
    The dial is rotated R14 to point at 14.
    The dial is rotated L82 to point at 32.
*/

pub fn turn(current: usize, turn: impl AsRef<str>) -> usize {
    let sign = &turn.as_ref()[0..1];
    let clicks = &turn.as_ref()[1..];
    let clicks = clicks.parse::<isize>().expect("parse click");
    let sign = if sign == "R" { 1 } else { -1isize };
    let num = current as isize + clicks * sign;
    dbg!(current, sign, clicks, num);
    let res = if num < 0 {
        // num is negative
        (99 + num) as usize
    } else if num > 99 {
        (num - 99) as usize
    } else {
        num as usize
    };
    dbg!(res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_turns() {
        let start = 50;
        let actual = turn(start, "L68");
        assert_eq!(actual, 82);
        let actual = turn(actual, "L68");
        assert_eq!(actual, 82);
        let actual = turn(actual, "L30");
        assert_eq!(actual, 52);
        let actual = turn(actual, "R48");
        assert_eq!(actual, 0);
        let actual = turn(actual, "L5");
        assert_eq!(actual, 95);
        let actual = turn(actual, "R60");
        assert_eq!(actual, 55);
        let actual = turn(actual, "L55");
        assert_eq!(actual, 0);
        let actual = turn(actual, "L1");
        assert_eq!(actual, 99);
        let actual = turn(actual, "L99");
        assert_eq!(actual, 0);
        let actual = turn(actual, "R14");
        assert_eq!(actual, 14);
        let actual = turn(actual, "L82");
        assert_eq!(actual, 32);
    }
}
