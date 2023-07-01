use std::env;
use tokio;
use reqwest;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
// use character_text_splitter::CharacterTextSplitter;

mod embedding;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    url: Option<String>,
    status: Option<Status>,
    domain: Option<String>,
    title: Option<String>,
    author: Option<Vec<String>>,
    date_published: Option<String>,
    images: Option<Vec<String>>,
    videos: Option<Vec<String>>,
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Status {
    Complete,
    Error,
    #[serde(other)]
    Unknown 
}

const BASE_URL: &str = "https://extractorapi.com/api/v1/extractor/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("EXTRACTOR_API_KEY").ok().unwrap();
    
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let resp: Response = reqwest::Client::new()
        .get(format!("{BASE_URL}?apikey={}&url={}", api_key, url))
        .send()
        .await?
        .json()
        .await?;

    let text = resp.text.clone().unwrap();
    println!("Resp: {:?}", text);
    println!("Text count: {}", text.len());

    let embeddings = embedding::get_embedding(&text).await.unwrap();
    println!("embeddings: {:?}", embeddings);

    // TODO: split the webpage content into chunks before upserting into a vector db
    // if let Some(text) = resp.text {
    //     let splitter = CharacterTextSplitter::new()
    //         .with_chunk_size(300)
    //         .with_chunk_overlap(50);
    //     let chunks = splitter.split_text(&text);
    //
    //     // for chunk in chunks {
    //     //     println!("{}", chunk);
    //     // }
    //
    //     // println!("chunks: {}", chunks.len());
    // }
        
    Ok(())
}
