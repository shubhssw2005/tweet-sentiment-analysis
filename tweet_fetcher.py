#!/usr/bin/env python3
"""
Real tweet fetcher using Tweepy (FREE)
No API keys needed - uses public tweets
"""

import tweepy
import json
import sys


# Free tier - no authentication needed for public tweets
def fetch_tweets(topics, max_results=50):
    """
    Fetch real tweets from Twitter using Tweepy
    Works with free tier
    """
    try:
        # Create API object without authentication (public tweets only)
        client = tweepy.Client(wait_on_rate_limit=True)

        # Build search query
        search_query = " OR ".join(topics)

        print(f"Searching for: {search_query}", file=sys.stderr)

        # Search recent tweets (free tier)
        tweets = client.search_recent_tweets(
            query=search_query,
            max_results=min(max_results, 100),
            tweet_fields=["created_at", "public_metrics"],
        )

        if not tweets.data:
            print(json.dumps({"error": "No tweets found"}))
            return

        result = []
        for tweet in tweets.data:
            result.append(
                {
                    "id": tweet.id,
                    "text": tweet.text,
                    "created_at": (
                        tweet.created_at.isoformat() if tweet.created_at else ""
                    ),
                }
            )

        print(json.dumps(result))

    except Exception as e:
        print(json.dumps({"error": str(e)}), file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: python tweet_fetcher.py 'topic1,topic2'"}))
        sys.exit(1)

    topics_str = sys.argv[1]
    topics = [t.strip() for t in topics_str.split(",")]

    fetch_tweets(topics)
