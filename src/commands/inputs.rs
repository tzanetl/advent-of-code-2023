use scraper::{Html, Selector};

use super::client::{build_with_defaults, AocClient};

pub async fn load_inputs(year: u32, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let client = build_with_defaults();
    load_test_input(&client, &year, &day).await?;
    Ok(())
}

async fn load_personal_input(
    client: &AocClient,
    year: &u32,
    day: &u32,
) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}

async fn load_test_input(
    client: &AocClient,
    year: &u32,
    day: &u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let resp = client.get(url).send().await?;

    let document = Html::parse_document(&resp.text().await?);
    let selector = Selector::parse("pre>code")?;
    let example_input = document
        .select(&selector)
        .next()
        .expect("No code node found")
        .inner_html();
    std::fs::write("test.txt", example_input)?;
    Ok(())
}
