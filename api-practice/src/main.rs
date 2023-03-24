#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get("https://www.google.co.jp/").await?.text().await?;
    println!("{:?}", res);
    Ok(())
}
