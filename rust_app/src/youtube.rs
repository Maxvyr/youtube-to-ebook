use crate::models::Video;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

pub struct YouTubeClient {
    client: Client,
    api_key: String,
}

impl YouTubeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            api_key,
        }
    }

    pub async fn get_latest_videos(&self, channels: &[&str]) -> Result<Vec<Video>> {
        let mut videos = Vec::new();

        for channel in channels {
            println!("Looking up: {}", channel);
            match self.process_channel(channel).await {
                Ok(Some(video)) => {
                    println!("  ✓ Found: {}", video.title);
                    videos.push(video);
                }
                Ok(None) => println!("  ✗ No long-form videos found"),
                Err(e) => println!("  ✗ Error processing channel {}: {}", channel, e),
            }
        }

        Ok(videos)
    }

    async fn process_channel(&self, channel_handle: &str) -> Result<Option<Video>> {
        let handle = channel_handle.trim_start_matches('@');
        
        // 1. Get Channel Info & Uploads Playlist
        let uploads_id = self.get_uploads_playlist_id(handle).await?;
        
        // 2. Get latest videos from playlist
        let playlist_items = self.get_playlist_items(&uploads_id).await?;

        // 3. Find first non-short video
        for item in playlist_items {
            let video_id = item.snippet.resource_id.video_id;
            if !self.is_short(&video_id).await {
                return Ok(Some(Video {
                    id: video_id.clone(),
                    title: item.snippet.title,
                    description: item.snippet.description,
                    channel_name: item.snippet.channel_title,
                    url: format!("https://www.youtube.com/watch?v={}", video_id),
                    transcript: None,
                }));
            }
        }

        Ok(None)
    }

    async fn get_uploads_playlist_id(&self, handle: &str) -> Result<String> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/channels?part=contentDetails&forHandle={}&key={}",
            handle, self.api_key
        );

        let resp: ChannelResponse = self.client.get(&url).send().await?.json().await?;
        
        let item = resp.items.first().context("Channel not found")?;
        Ok(item.content_details.related_playlists.uploads.clone())
    }

    async fn get_playlist_items(&self, playlist_id: &str) -> Result<Vec<PlaylistItem>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&playlistId={}&maxResults=15&key={}",
            playlist_id, self.api_key
        );

        let resp: PlaylistResponse = self.client.get(&url).send().await?.json().await?;
        Ok(resp.items)
    }

    async fn is_short(&self, video_id: &str) -> bool {
        let url = format!("https://www.youtube.com/shorts/{}", video_id);
        // We make a HEAD request and check if the final URL still contains /shorts/
        // Note: reqwest follows redirects by default.
        match self.client.head(&url).send().await {
            Ok(resp) => resp.url().as_str().contains("/shorts/"),
            Err(_) => false, // Assume not a short if error (safest fallback? or maybe true? logic in python was: error -> False)
        }
    }
}

// --- API Response Structs ---

#[derive(Deserialize)]
struct ChannelResponse {
    items: Vec<ChannelItem>,
}

#[derive(Deserialize)]
struct ChannelItem {
    #[serde(rename = "contentDetails")]
    content_details: ContentDetails,
}

#[derive(Deserialize)]
struct ContentDetails {
    #[serde(rename = "relatedPlaylists")]
    related_playlists: RelatedPlaylists,
}

#[derive(Deserialize)]
struct RelatedPlaylists {
    uploads: String,
}

#[derive(Deserialize)]
struct PlaylistResponse {
    #[serde(default)]
    items: Vec<PlaylistItem>,
}

#[derive(Deserialize)]
struct PlaylistItem {
    snippet: Snippet,
}

#[derive(Deserialize)]
struct Snippet {
    title: String,
    description: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    #[serde(rename = "resourceId")]
    resource_id: ResourceId,
}

#[derive(Deserialize)]
struct ResourceId {
    #[serde(rename = "videoId")]
    video_id: String,
}
