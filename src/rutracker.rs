use serde_json::Value;

#[tokio::main]
pub async fn search(value: &str) -> Result<Value, reqwest::Error> {
    let api_url = "https://api.michijackson.xyz/search?q=".to_owned();
    let results = reqwest::get(api_url + value).await?;
    let text = results.text().await?;
    let v: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
    let data = &v["data"];
    println!("{:?}", data);
    Ok(v)
}
