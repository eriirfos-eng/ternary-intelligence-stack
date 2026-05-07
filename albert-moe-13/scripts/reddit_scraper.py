import praw
import json
import os
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

REDDIT_CLIENT_ID = os.getenv('REDDIT_CLIENT_ID')
REDDIT_CLIENT_SECRET = os.getenv('REDDIT_CLIENT_SECRET')
REDDIT_USER_AGENT = 'AlbertAgenticDataCollector/0.1'

def fetch_resolved_threads(subreddit_name, limit=10):
    if not REDDIT_CLIENT_ID or not REDDIT_CLIENT_SECRET:
        print("Error: Reddit API credentials not found in environment variables.")
        return []

    reddit = praw.Reddit(
        client_id=REDDIT_CLIENT_ID,
        client_secret=REDDIT_CLIENT_SECRET,
        user_agent=REDDIT_USER_AGENT
    )
    
    subreddit = reddit.subreddit(subreddit_name)
    resolved_data = []

    # Searching for threads that contain "solved" or "fixed"
    search_query = 'flair:solved OR "fixed it" OR "solved" OR "solution"'
    
    for submission in subreddit.search(search_query, limit=limit, sort='relevance'):
        thread = {
            "title": submission.title,
            "url": submission.url,
            "score": submission.score,
            "num_comments": submission.num_comments,
            "body": submission.selftext,
            "comments": []
        }
        
        if submission.num_comments > 5:
            submission.comments.replace_more(limit=0)
            for comment in submission.comments.list():
                thread["comments"].append({
                    "author": str(comment.author),
                    "body": comment.body,
                    "score": comment.score
                })
            resolved_data.append(thread)
            
    return resolved_data

if __name__ == "__main__":
    print("Initiating secure Reddit Scraper...")
    # Example: run the fetch if credentials exist
    # data = fetch_resolved_threads('learnprogramming', limit=2)
    # print(f"Collected {len(data)} threads.")
