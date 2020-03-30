use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct WOTD {
    word: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = fs::read_to_string(".env").expect("Something went wrong reading the file");
    let api_key: Vec<&str> = env.split("api_key=").collect();
    let api_key: String = api_key[1].to_string();
    let url = format!("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key={}", api_key);
    let resp = get(&url).await?.text().await?;
    let wotd: WOTD = serde_json::from_str(&resp).unwrap();
    println!("Word of the Day is: {}", wotd.word);
    Ok(())
}
