use crate::models::{Video, Article};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ClaudeClient {
    client: Client,
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn generate_article(&self, video: &Video) -> Result<String> {
        let transcript = video.transcript.as_ref().context("No transcript available")?;

        let prompt = format!(
            "You are a skilled magazine writer. Transform this YouTube video transcript into a well-written, engaging article.

VIDEO TITLE: {}
CHANNEL: {}
VIDEO URL: {}

VIDEO DESCRIPTION:
{}

TRANSCRIPT:
{}

---

Remix this YouTube transcript into a magazine article. Guidelines:
- Use the video title and description to correct any transcription errors, especially names of people, companies, or technical terms. The description often contains the correct spellings.
- Start with an engaging headline (different from the video title)
- The audience is a curious individual who is generally smart but not a specialist or expert in the area mentioned in the video
- Highly engaging and readable. Wherever jargon or obscure references appear, explain them. Extremely well-written; think New Yorker or the Atlantic
- Capture the key insights, especially contrarian viewpoints, memorable anecdotes, and surprising insights. Preserve key quotes (clean up filler words or transcription errors).
- There's no fixed length requirement; it depends on the length of the original article as well as the insight density. Make your own judgment. This should be a satisfying long-read.
- Do NOT include phrases like \"In this video\" - write it as a standalone article. Assume the reader has not watched the video and has zero context about it. This article is meant to be as a replacement, not complement, for watching the video.

Format the article in clean markdown.",
            video.title,
            video.channel_name,
            video.url,
            video.description,
            transcript
        );

        let request = CompletionRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 8192,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let resp = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await?;
            anyhow::bail!("API Error {}: {}", status, text);
        }

        let resp_body: CompletionResponse = resp.json().await?;

        Ok(resp_body.content[0].text.clone())
    }
}

// --- API Structs ---

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct CompletionResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}
