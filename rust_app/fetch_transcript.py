import sys
from youtube_transcript_api import YouTubeTranscriptApi

def get_transcript(video_id):
    try:
        # Create an instance of the API (newer version syntax)
        ytt_api = YouTubeTranscriptApi()
        
        # Try fetching English or French
        transcript_list = ytt_api.fetch(video_id, languages=['en', 'fr'])
        
        full_text = ""
        for segment in transcript_list:
            full_text += segment.text + " "
            
        print(full_text.strip())
        sys.exit(0)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python fetch_transcript.py <VIDEO_ID>", file=sys.stderr)
        sys.exit(1)
        
    video_id = sys.argv[1]
    get_transcript(video_id)
