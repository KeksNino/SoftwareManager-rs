use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Software {
    pub author: String,
    pub leechers: String,
    pub seeders: String,
    pub title: String,
    pub url: String,
}

#[tokio::main]
pub async fn search(value: &str) -> Result<Value, reqwest::Error> {
    let api_url = "https://api.michijackson.xyz/search?q=".to_owned();
    let results = reqwest::get(api_url + value).await?;
    let text = results.text().await?;
    let v: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
    let data = &v["data"];
    for item in data.as_array().unwrap() {
        let software: Software =
            serde_json::from_value(item.clone()).expect("Failed to deserialize");
        println!("{:?}", software);
    }
    println!("{:?}", data);
    Ok(v)
}
