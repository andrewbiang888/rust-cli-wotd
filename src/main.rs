use std::thread;
use std::time::Duration;
use std::fs;
use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use structopt::StructOpt;
use indicatif::ProgressBar;

#[derive(Debug, Serialize, Deserialize)]
struct WOTD {
    word: String,
    id: u32,
}

#[derive(Debug, StructOpt)]
struct Cli {
    arg: String,
    word: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = fs::read_to_string(".env").expect("Something went wrong reading the file");
    let api_key: Vec<&str> = env.split("api_key=").collect();
    let api_key: String = api_key[1].to_string();
    let args = Cli::from_args();
    let command = args.arg;
    
    match &command[..] {
        "get" => {
            let pb = ProgressBar::new(100);
            let url = format!("https://api.wordnik.com/v4/words.json/randomWord?hasDictionaryDef=true&minLength=5&maxLength=-1&api_key={}", api_key);
            let resp = get(&url).await?.text().await?;
            for _ in 0..100 {
                pb.inc(1);
                thread::sleep(Duration::from_millis(5));
            }
            let wotd: WOTD = serde_json::from_str(&resp).unwrap();
            pb.finish_and_clear();
            println!("Word of the Day is: {}", wotd.word);
        }
        "define" => {
            let pb = ProgressBar::new(100);
            let lookup = match args.word {
                Some(val) => val,
                None => String::from("nil"),
            };
            if lookup == String::from("nil") {
                println!("To define a word, please provide one. Ex: \"wotd define sup\"");
                return Ok(())
            }
            let url = format!("https://api.wordnik.com/v4/word.json/{}/definitions?limit=1&sourceDictionaries=webster&includeRelated=false&useCanonical=false&includeTags=false&api_key={}", lookup, api_key);
            let resp = get(&url).await?.text().await?;
            for _ in 0..100 {
                pb.inc(1);
                thread::sleep(Duration::from_millis(5));
            }
            let word_res: Value = serde_json::from_str(&resp).unwrap();
            pb.finish_and_clear();
            println!("The definition of {} is: {}", lookup, word_res[0]["text"]);
        }
        _ => println!("{} is not a supported wotd command.", command),
    }

    Ok(())
}
