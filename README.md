# Scraper

### What
This tool scrapes a webpage, extracts its contents, and convert them into vector representation using Vertex AI's embeddings api by Google. 

### Why
I wanted to add a web scraping capability to my GPT-4-based Slack bot. The embedding api doesn't have to be the one by Vertex AI. I chose it because I haven't tried it before. 

### TODO
- Split the text into chunks before converting them into vector data
- Upsert the vector data into a vector db (e.g., Qdrant, Chroma)
