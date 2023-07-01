// Use Vertex API to get embedding data for a given text string
// https://cloud.google.com/vertex-ai/docs/generative-ai/embeddings/get-text-embeddings#generative-ai-get-text-embedding-drest

use std::env;
use reqwest::{self, header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddingRequest {
    instances: Vec<EmbeddingRequestContent>,
}

impl EmbeddingRequest {
    pub fn new(text: &str) -> Self {
        EmbeddingRequest {
            instances: vec![
                EmbeddingRequestContent {
                    text: text.to_string()
                }
            ]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddingRequestContent {
    #[serde(rename = "content")]
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddingResponse {
    predictions: Vec<Prediction>
}

#[derive(Debug, Serialize, Deserialize)]
struct Prediction {
    embeddings: Embeddings
}

#[derive(Debug, Serialize, Deserialize)]
struct Embeddings {
    statistics: Statistics,
    values: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Statistics {
    truncated: bool,
    token_count: i32,
}

pub async fn get_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let url = format!("https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/us-central1/publishers/google/models/textembedding-gecko:predict", env::var("GOOGLE_PROJECT_ID").ok().unwrap());
    
    let request_body = EmbeddingRequest::new(text);

    let mut headers = header::HeaderMap::new();
    let bearer = format!("Bearer {}", env::var("GOOGLE_ACCESS_TOKEN").ok().unwrap());
    headers.insert("Authorization", header::HeaderValue::from_str(&bearer).unwrap());
    headers.insert("Content-Type", header::HeaderValue::from_static("application/json; charset=utf-8"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    
    let res = client
        .post(url)
        .json(&request_body)
        .send()
        .await?;

    let js = res
        .json::<EmbeddingResponse>()
        .await?;

    let embedding = &js.predictions.first().unwrap().embeddings.values;
    Ok(embedding.to_vec())
}

