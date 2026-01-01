pub fn max_jolts(bank: impl AsRef<str>) -> usize {
    let bank = bank.as_ref();
    let (first, idx) = largest(&bank[..bank.len() - 1]);
    let (second, _idx) = largest(&bank[idx + 1..]);
    format!("{first}{second}").parse().expect("parsing sum")
}

fn largest(digits: &str) -> (usize, usize) {
    for num in ('1'..='9').rev() {
        let found = digits.find(num);
        match found {
            Some(idx) => {
                return (num.to_digit(10).unwrap() as usize, idx);
            }
            None => continue,
        }
    }
    unreachable!("input contains only digits");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_max_bateries() {
        // In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
        let battery = max_jolts("987654321111111");
        assert_eq!(battery, 98);
        // In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
        let battery = max_jolts("811111111111119");
        assert_eq!(battery, 89);
        // In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
        let battery = max_jolts("234234234234278");
        assert_eq!(battery, 78);
        // In 818181911112111, the largest joltage you can produce is 92.
        let battery = max_jolts("818181911112111");
        assert_eq!(battery, 92);
    }
}
