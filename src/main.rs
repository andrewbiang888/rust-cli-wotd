use reqwest::{get, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WOTD {
    word: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let resp = get("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key=5vl8hqmmi8dw4lwe13jcd5zf0q3cenlh1zqci78gk2jp1wdbj").await?.text().await?;
    let wotd: WOTD = serde_json::from_str(&resp).unwrap();
    println!("{:#?}", wotd.word);
    Ok(())
}
