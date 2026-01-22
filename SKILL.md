---
name: youtube-to-ebook
description: Transforms YouTube video transcripts into magazine-style EPUB ebooks. Use when the user wants to convert YouTube videos to readable articles, generate ebooks from channel content, extract and reformat video transcripts, or set up a YouTube-to-ebook pipeline.
---

# YouTube to Ebook

Transform YouTube videos from this channels into well-written magazine-style articles, delivered as an EPUB ebook.

## What This Skill Does

1. Fetches latest videos from YouTube channels (filtering out Shorts)
2. Extracts transcripts from those videos
3. Transforms transcripts into polished articles using Claude
4. Packages articles into an EPUB ebook for reading on any device

## Quick Start

Ask: "Set up YouTube to ebook for me"

Claude will guide you through:
1. Creating a project folder
2. Setting up YouTube API access
3. Configuring your favorite channels
4. Generating your first ebook

## Requirements

- Python 3.8+

## Commands

| Command | Description |
|---------|-------------|
| `python main.py` | Generate ebook from latest videos |
| `python main.py --channels` | Edit channel list |
| `python dashboard.py` | Launch web dashboard |

## Key Files

```
youtube-to-ebook/
├── get_videos.py      # Fetch latest videos and channel list
├── get_transcripts.py # Extract transcripts
├── write_articles.py  # Transform to articles
├── send_email.py      # Create EPUB & send
├── main.py            # Run full pipeline
└── .env               # API keys
```

## Customization

### Writing Style
Edit the prompt in `write_articles.py` to change article tone:
- Magazine style (default)
- Academic summary
- Casual blog post
- Technical documentation

## Workflow

```
┌─────────────┐    ┌──────────────┐    ┌───────────────┐    ┌────────────┐
│ Fetch Videos│───▶│Get Transcripts│───▶│Write Articles │───▶│Create EPUB │
│ (YouTube API)│    │(Transcript API)│    │  (Claude AI)  │    │ (ebooklib) │
└─────────────┘    └──────────────┘    └───────────────┘    └────────────┘
```

## Example Output

The generated EPUB contains:
- Table of contents with all articles
- Clean, readable formatting
- Original video links for reference
- Mobile-friendly styling
