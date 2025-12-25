use day_3::max_jolts;

fn main() {
    let battery_banks = inputs::fetch_input(3, None);
    let all_batteries = battery_banks.iter().map(max_jolts).sum();
    dbg!(all_batteries);
}
