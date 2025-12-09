use day_1::turn;

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
