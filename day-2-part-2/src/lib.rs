use std::collections::HashMap;

pub fn invalid(range: &str, cache: &mut HashMap<usize, bool>) -> Vec<usize> {
    let (start, end) = range.split_once('-').expect("expected '-' in range");
    let start = start.parse::<usize>().expect("start is not a usize");
    let end = end.parse::<usize>().expect("end is not a usize");

    (start..=end)
        .filter_map(|num| {
            if *cache.entry(num).or_insert(is_invalid(num)) {
                Some(num)
            } else {
                None
            }
        })
        .collect()
}

fn is_invalid(num: usize) -> bool {
    let num = num.to_string();
    if num.len() % 2 != 0 {
        false
    } else {
        let half = num.len() / 2;
        &num[..half] == &num[half..]
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::invalid;

    #[test]
    fn find_invalid_ids() {
        // 11-22 still has two invalid IDs, 11 and 22.
        let ids = invalid("11-22", &mut HashMap::new());
        assert_eq!(ids, &[11, 22]);
        // 95-115 now has two invalid IDs, 99 and 111.
        let ids = invalid("95-115", &mut HashMap::new());
        assert_eq!(ids, &[99, 111]);
        // 998-1012 now has two invalid IDs, 999 and 1010.
        let ids = invalid("998-1012", &mut HashMap::new());
        assert_eq!(ids, &[999, 1010]);
        // 1188511880-1188511890 still has one invalid ID, 1188511885.
        let ids = invalid("1188511880-1188511890", &mut HashMap::new());
        assert_eq!(ids, &[1188511885]);
        // 222220-222224 still has one invalid ID, 222222.
        let ids = invalid("222220-222224", &mut HashMap::new());
        assert_eq!(ids, &[222222]);
        // 1698522-1698528 still contains no invalid IDs.
        let ids = invalid("1698522-1698528", &mut HashMap::new());
        assert_eq!(ids, &[]);
        // 446443-446449 still has one invalid ID, 446446.
        let ids = invalid("446443-446449", &mut HashMap::new());
        assert_eq!(ids, &[446446]);
        // 38593856-38593862 still has one invalid ID, 38593859.
        let ids = invalid("38593856-38593862", &mut HashMap::new());
        assert_eq!(ids, &[38593859]);
        // 565653-565659 now has one invalid ID, 565656.
        let ids = invalid("565653-565659", &mut HashMap::new());
        assert_eq!(ids, &[565656]);
        // 824824821-824824827 now has one invalid ID, 824824824.
        let ids = invalid("824824821-824824827", &mut HashMap::new());
        assert_eq!(ids, &[824824824]);
        // 2121212118-2121212124 now has one invalid ID, 2121212121.
        let ids = invalid("2121212118-2121212124", &mut HashMap::new());
        assert_eq!(ids, &[2121212121]);
    }
}
