use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = env::var("SESSION").expect("SESSION environment variable not set");
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&format!("session={}", session))?,
            );
            headers
        })
        .build()?;

    let years = vec![2019];
    let days = 1..=25;

    // Iterate over years and days to fetch inputs
    for year in &years {
        for day in days.clone() {
            let input_path = Path::new("src")
                .join("inputs")
                .join(format!("day_{}.txt", day));

            if input_path.exists() {
                continue;
            }

            let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let input = client.get(&url).send()?.text()?;

            fs::create_dir_all(input_path.parent().unwrap())?;
            fs::write(&input_path, input)?;

            println!("Fetched and saved input for year {} day {}.", year, day);
        }
    }

    Ok(())
}
