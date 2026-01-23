use anyhow::{Context, Result};
use std::process::Command;

pub struct TranscriptFetcher;

impl TranscriptFetcher {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_transcript(&self, video_id: &str) -> Result<String> {
        // Call the Python script helper
        // Assuming fetch_transcript.py is in the current directory (where cargo run is executed)
        let output = Command::new("python3")
            .arg("fetch_transcript.py")
            .arg(video_id)
            .output()
            .context("Failed to execute python script for transcript")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python script error: {}", stderr.trim());
        }

        let full_text = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in transcript output")?;

        Ok(full_text.trim().to_string())
    }
}
