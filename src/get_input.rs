use std::{env, error::Error, fs};

use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let session = env!("SESSION");

    let args: Vec<_> = env::args().collect();
    let day = args[1].parse::<u8>()?;

    let client = Client::builder().build()?;
    let request = client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header("Cookie", format!("session={session}"))
        .build()?;
    let input = client.execute(request)?.text()?;
    fs::create_dir_all(format!("day{day:02}/"))?;
    fs::write(format!("day{day:02}/input.txt"), input)?;
    Ok(())
}
