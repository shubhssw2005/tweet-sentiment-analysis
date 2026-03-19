#!/usr/bin/env python3
"""
Tweet scraper using ZenDriver for Linux
Undetectable browser automation with proxy rotation and IP changing
"""
import json
import sys
import asyncio
import subprocess
import random


def install_deps():
    """Install required packages"""
    packages = ["zendriver", "requests"]
    for package in packages:
        try:
            __import__(package.replace("-", "_"))
        except ImportError:
            print(f"Installing {package}...", file=sys.stderr)
            subprocess.check_call(
                [sys.executable, "-m", "pip", "install", package, "-q"]
            )


# Proxy list for IP rotation
PROXIES = [
    "http://proxy1.example.com:8080",
    "http://proxy2.example.com:8080",
    "socks5://127.0.0.1:9050",  # Tor
]


def get_random_proxy():
    """Get random proxy"""
    return random.choice(PROXIES) if PROXIES else None


async def get_tweets_zendriver(topic, proxy=None):
    """
    Fetch tweets for a single topic using ZenDriver
    Works on Linux with Chrome/Chromium
    """
    try:
        from zendriver import start, Config

        print(f"ZenDriver: Fetching {topic} (proxy: {proxy})", file=sys.stderr)

        # Create config for Linux
        config = Config(
            headless=True,
            no_sandbox=True,  # Required on Linux
            disable_blink_features="AutomationControlled",
        )

        if proxy:
            config.proxy = proxy

        # Start browser
        browser = await start(config=config)
        print(f"Browser started for {topic}", file=sys.stderr)

        # Get tab
        tab = await browser.get()

        # Navigate to Twitter search
        search_url = f"https://x.com/search?q={topic}&f=live"
        print(f"Navigating to: {search_url}", file=sys.stderr)
        await tab.goto(search_url)

        # Wait for tweets to load
        await asyncio.sleep(6)

        # Scroll to load more tweets
        await tab.execute_script("window.scrollBy(0, window.innerHeight);")
        await asyncio.sleep(2)

        # Find tweet elements
        tweets = []
        tweet_elements = await tab.find_elements("//article[@data-testid='tweet']")
        print(f"Found {len(tweet_elements)} tweets for {topic}", file=sys.stderr)

        for i, element in enumerate(tweet_elements[:15]):
            try:
                # Get tweet text
                text_elem = await element.find_element(
                    ".//div[@data-testid='tweetText']"
                )
                text = await text_elem.text

                if text and len(text) > 10:
                    tweets.append({"text": text, "topic": topic, "source": "zendriver"})
                    print(f"Tweet {i+1}: OK", file=sys.stderr)
            except Exception as e:
                print(f"Error extracting tweet {i+1}: {e}", file=sys.stderr)
                pass

        await browser.close()
        print(f"Browser closed for {topic}", file=sys.stderr)

        return tweets

    except Exception as e:
        print(f"ZenDriver error for {topic}: {e}", file=sys.stderr)
        import traceback

        traceback.print_exc()
        return []


async def get_tweets_concurrent(topics, num_threads=3):
    """
    Fetch tweets concurrently with thread pool
    Each topic gets a different proxy for IP rotation
    """
    all_tweets = []

    # Create tasks for each topic with different proxies
    tasks = [get_tweets_zendriver(topic, proxy=get_random_proxy()) for topic in topics]

    # Run concurrently
    results = await asyncio.gather(*tasks, return_exceptions=True)

    for result in results:
        if isinstance(result, list):
            all_tweets.extend(result)
        else:
            print(f"Task error: {result}", file=sys.stderr)

    return all_tweets


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(json.dumps({"error": "No topics provided"}))
        sys.exit(1)

    install_deps()

    topics = [t.strip() for t in sys.argv[1].split(",")]
    num_threads = int(sys.argv[2]) if len(sys.argv) > 2 else 3

    # Run async function
    tweets = asyncio.run(get_tweets_concurrent(topics, num_threads=num_threads))

    if tweets:
        print(json.dumps(tweets))
    else:
        print(json.dumps({"error": "No tweets found"}))
