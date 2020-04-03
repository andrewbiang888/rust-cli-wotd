use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct WOTD {
    word: String,
    id: u32,
}

#[derive(Debug, StructOpt)]
struct Cli {
    arg: String,
    word: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = fs::read_to_string(".env").expect("Something went wrong reading the file");
    let api_key: Vec<&str> = env.split("api_key=").collect();
    let api_key: String = api_key[1].to_string();
    let args = Cli::from_args();
    let command = args.arg;
    let lookup = args.word;
    match &command[..] {
        "get" => {
            let url = format!("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key={}", api_key);
            let resp = get(&url).await?.text().await?;
            let wotd: WOTD = serde_json::from_str(&resp).unwrap();
            println!("Word of the Day is: {}", wotd.word);
        }
        "define" => {
            let url = format!("https://api.wordnik.com/v4/word.json/{}/definitions?limit=1&sourceDictionaries=webster&includeRelated=false&useCanonical=false&includeTags=false&api_key={}", lookup, api_key);
            let resp = get(&url).await?.text().await?;
            let word_res: Value = serde_json::from_str(&resp).unwrap();
            println!("The definition of {} is: {}", lookup, word_res[0]["text"]);
        }
        _ => println!("{} is not a supported wotd command.", command),
    }

    Ok(())
}
