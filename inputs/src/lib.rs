pub fn fetch_input(day: usize, part: Option<usize>) -> Vec<String> {
    let mut inputs_path = format!("./day-{day}");
    if let Some(part) = part {
        inputs_path.push_str(&format!("-part-{part}"));
    };
    let path = std::path::absolute(&inputs_path);
    println!("{path:?}");
    let inputs_txt = format!("./{inputs_path}/inputs.txt");
    let input = match std::fs::read_to_string(&inputs_txt) {
        Ok(input) if !input.is_empty() => {
            println!("Using cached input");
            input
        }
        _ => {
            let session = std::env::var("ADVENT_SESSION").expect("missing $ADVENT_SESSION token");
            const BASE_URL: &str = "https://adventofcode.com/2025/day";
            let url = format!("{BASE_URL}/{day}/input");
            let client = reqwest::blocking::Client::new();
            let input = client
                .get(url)
                .header("Cookie", format!("session={session}"))
                .send()
                .expect("fetching inputs")
                .text()
                .expect("read body as text");
            std::fs::write(inputs_txt, &input).expect("writing to cache");
            input
        }
    };
    input.lines().map(&str::to_string).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fetch_input(1, None);
        assert_eq!(result[0], "L50");
    }
}
