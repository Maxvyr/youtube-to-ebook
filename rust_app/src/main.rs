mod models;
mod youtube;
mod transcript;
mod claude;
mod epub;
mod email;

use crate::models::{Article, Video};
use crate::youtube::YouTubeClient;
use crate::transcript::TranscriptFetcher;
use crate::claude::ClaudeClient;
use crate::epub::EpubGenerator;
use crate::email::EmailSender;
use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

// Configuration
const CHANNELS: &[&str] = &[
    "@aliabdaal",
    "@t3dotgg",
    "@AlexFinnOfficial",
    "@maximevidalinc"
];

#[tokio::main]
async fn main() -> Result<()> {
    // 0. Setup
    dotenv().ok();
    println!("{}", "=".repeat(60));
    println!("  YOUTUBE NEWSLETTER GENERATOR (RUST VERSION)");
    println!("{}", "=".repeat(60));

    let youtube_key = env::var("YOUTUBE_API_KEY").context("YOUTUBE_API_KEY not found")?;
    let claude_key = env::var("ANTHROPIC_API_KEY").context("ANTHROPIC_API_KEY not found")?;

    let yt_client = YouTubeClient::new(youtube_key);
    let transcript_fetcher = TranscriptFetcher::new();
    let claude_client = ClaudeClient::new(claude_key);

    // 1. Fetch Videos
    println!("\n📺 STEP 1: Fetching latest videos...\n");
    let videos = yt_client.get_latest_videos(CHANNELS).await?;

    if videos.is_empty() {
        println!("No videos found.");
        return Ok(());
    }

    println!("\n  → {} video(s) to process\n", videos.len());

    // 2. Fetch Transcripts
    println!("\n📝 STEP 2: Extracting transcripts...\n");
    let mut videos_with_transcripts = Vec::new();
    
    for mut video in videos {
        print!("Getting transcript: {}... ", video.title);
        match transcript_fetcher.fetch_transcript(&video.id).await {
            Ok(text) => {
                println!("✓ ({} chars)", text.len());
                video.transcript = Some(text);
                videos_with_transcripts.push(video);
            },
            Err(e) => {
                println!("✗ Error: {}", e);
            }
        }
    }

    if videos_with_transcripts.is_empty() {
        println!("No transcripts available for any video.");
        return Ok(());
    }

    // 3. Write Articles
    println!("\n✍️ STEP 3: Writing articles with Claude AI...\n");
    let mut articles = Vec::new();

    for video in &videos_with_transcripts {
        println!("Writing article for: {}...", video.title);
        match claude_client.generate_article(video).await {
            Ok(content) => {
                println!("  ✓ Article generated!");
                articles.push(Article {
                    title: video.title.clone(),
                    channel: video.channel_name.clone(),
                    url: video.url.clone(),
                    content,
                });
            },
            Err(e) => println!("  ✗ Failed to generate article: {}", e),
        }
    }

    if articles.is_empty() {
        println!("No articles generated.");
        return Ok(());
    }

    // 4. Generate EPUB
    println!("\n📚 STEP 4: Creating EPUB...\n");
    let output_file = "newsletter.epub";
    EpubGenerator::create_epub(&articles, output_file)?;
    println!("  ✓ Saved to {}", output_file);

    // 5. Send Email
    println!("\n📧 STEP 5: Sending Email...\n");
    // Optional: Get credentials. If missing, skip email.
    if let (Ok(email_addr), Ok(email_pass)) = (env::var("GMAIL_ADDRESS"), env::var("GMAIL_APP_PASSWORD")) {
        println!("  Sending to {}...", email_addr);
        let sender = EmailSender::new(email_addr.clone(), email_pass);
        match sender.send_newsletter(&email_addr, &articles, output_file).await {
            Ok(_) => println!("  ✓ Email sent successfully!"),
            Err(e) => println!("  ✗ Failed to send email: {}", e),
        }
    } else {
        println!("  (Skipping email: GMAIL_ADDRESS or GMAIL_APP_PASSWORD not set)");
    }

    println!("{}", "=".repeat(60));
    println!("  DONE!");
    println!("{}", "=".repeat(60));

    Ok(())
}
