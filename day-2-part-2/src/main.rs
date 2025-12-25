use std::collections::HashMap;

use day_2_part_2::invalid;
use inputs::fetch_input;

fn main() {
    let inputs = fetch_input(2, None);
    let mut cache = HashMap::new();
    let mut total = 0;
    // ranges are not on single line as it is mentionded in the describtion
    for range in inputs[0].split(',') {
        let invalids = invalid(range, &mut cache);
        let invalids_sum = invalids.iter().sum::<usize>();
        total += invalids_sum;
    }
    dbg!(total);
}
