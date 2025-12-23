use day_1::turn;

fn main() {
    let inputs = inputs::fetch_input(1, None);

    let mut current = 50;

    let mut zeros = 0;
    for clicks in &inputs {
        // dbg!(clicks);
        println!("current: {current}, clicks: {clicks}");
        current = turn(current, clicks);
        println!("new current: {current}");
        if current == 0 {
            println!("#### inc zeros from {zeros} to {}", zeros + 1);
            zeros += 1;
        }
    }
    dbg!(zeros);
}
