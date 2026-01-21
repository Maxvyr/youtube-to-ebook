# How to Share This Skill on X

## Option 1: Share the GitHub Repository (Recommended)

1. Create a GitHub repository:
   ```bash
   cd ~/youtube-newsletter
   git init
   git add .
   git commit -m "YouTube to Ebook - Claude Skill"
   ```

2. Create a new repo on github.com, then:
   ```bash
   git remote add origin https://github.com/YOUR_USERNAME/youtube-to-ebook.git
   git push -u origin main
   ```

3. Post on X with the link

## Option 2: Share as a Gist

1. Go to https://gist.github.com
2. Paste the contents of SKILL.md
3. Create a public gist
4. Share the gist URL

---

## Sample X Posts

### Short Version (280 chars):
```
Built a Claude skill that turns YouTube videos into ebooks 📚

• Fetches latest videos from your fav channels
• Extracts transcripts
• AI writes magazine-style articles
• Outputs clean EPUB

Free to use: [YOUR_LINK]

#ClaudeAI #AI #YouTube
```

### Thread Version:

**Tweet 1:**
```
I built a Claude skill that transforms YouTube videos into beautifully formatted ebooks 📚🎬

Here's how it works 🧵
```

**Tweet 2:**
```
The problem: I subscribe to great YouTube channels but don't always have time to watch.

The solution: Auto-convert videos to magazine articles I can read anywhere.
```

**Tweet 3:**
```
What it does:
1. Fetches latest videos (filters out Shorts)
2. Grabs transcripts
3. Claude rewrites them as polished articles
4. Packages into EPUB ebook

All automated.
```

**Tweet 4:**
```
Key learnings from building this:

❌ YouTube Search API isn't chronological
✅ Use uploads playlist instead

❌ Shorts filtering by duration fails
✅ Check /shorts/ URL pattern

❌ Cloud automation gets blocked
✅ Run locally with launchd
```

**Tweet 5:**
```
The skill is free and open source.

It documents all the pitfalls I hit so you don't have to.

Link: [YOUR_LINK]

Built with @AnthropicAI Claude 🤖
```

---

## Files to Include in Your Share

Essential files for others to use:
- `SKILL.md` - The skill documentation
- `get_videos.py` - Fetch YouTube videos
- `get_transcripts.py` - Extract transcripts
- `write_articles.py` - Transform to articles
- `send_email.py` - Create EPUB (and optionally email)
- `main.py` - Run the pipeline
- `requirements.txt` - Python dependencies
- `.env.example` - Template for API keys

Create `.env.example` (without real keys):
```
YOUTUBE_API_KEY=your_youtube_api_key_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
GMAIL_ADDRESS=your_email@gmail.com
GMAIL_APP_PASSWORD=your_app_password_here
```
