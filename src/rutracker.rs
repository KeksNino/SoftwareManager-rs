#[tokio::main]
pub async fn search(value: &str) {
    let api_url = "https://api.michijackson.xyz/search?q=".to_owned();
    let results = reqwest::get(api_url + value).await;
    println!("{:?}", results);
}
