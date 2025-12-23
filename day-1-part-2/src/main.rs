use day_1_part_2::turn;

fn main() {
    let inputs = inputs::fetch_input(1, None);

    let mut current = 50;

    let mut zeros = 0;
    for clicks in &inputs {
        // dbg!(clicks);
        println!("current: {current}, clicks: {clicks}");
        let (cur, zs) = turn(current, clicks);
        current = cur;
        zeros += zs;
        println!("new current: {current}, zeros: {zeros}");
    }
    dbg!(zeros);
}
