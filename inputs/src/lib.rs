pub fn fetch_input(day: u8) -> Vec<String> {
    let session = std::env::var("ADVENT_SESSION").expect("missing $ADVENT_SESSION token");
    const INPUTS_CACHE: &str = "inputs.txt";
    let input = match std::fs::read_to_string(INPUTS_CACHE) {
        Ok(input) if !input.is_empty() => {
            println!("Using cached input");
            input
        }
        _ => {
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
            std::fs::write(INPUTS_CACHE, &input).expect("writing to cache");
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
        let result = fetch_input(1);
        assert_eq!(result[0], "L50");
    }
}
