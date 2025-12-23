use std::collections::HashMap;

pub fn invalid(range: &str, cache: &mut HashMap<usize, bool>) -> usize {
    let (start, end) = range.split_once('-').expect("expected '-' in range");
    let start = start.parse::<usize>().expect("start is not a usize");
    // drop semicolon
    let end = end
        .strip_suffix(";")
        .expect("end misses ;")
        .parse::<usize>()
        .expect("end is not a usize");

    (start..=end).fold(0, |total, num| {
        if *cache.entry(num).or_insert(is_invalid(num)) {
            total + num
        } else {
            total
        }
    })
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
        // 11-22 has two invalid IDs, 11 and 22.
        let invalids_sum = invalid("11-22;", &mut HashMap::new());
        assert_eq!(invalids_sum, 11 + 22);
        // 95-115 has one invalid ID, 99.
        // 998-1012 has one invalid ID, 1010.
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        // 222220-222224 has one invalid ID, 222222.
        // 1698522-1698528 contains no invalid IDs.
        // 446443-446449 has one invalid ID, 446446.
        // 38593856-38593862 has one invalid ID, 38593859.
        // The rest of the ranges contain no invalid IDs.
    }
}
