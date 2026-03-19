#!/usr/bin/env python3
"""
Test ZenDriver on Linux
Run this on a Linux server to verify ZenDriver works
"""
import asyncio
import subprocess
import sys


def install_zendriver():
    """Install zendriver"""
    subprocess.check_call([sys.executable, "-m", "pip", "install", "zendriver", "-q"])


async def test_zendriver():
    """Test ZenDriver"""
    try:
        from zendriver import start, Config

        print("=" * 60)
        print("Testing ZenDriver on Linux")
        print("=" * 60)

        print("\n1. Creating config...")
        config = Config(headless=True, no_sandbox=True)

        print("2. Starting browser...")
        browser = await start(config=config)
        print("   ✓ Browser started")

        print("3. Getting tab...")
        tab = await browser.get()
        print("   ✓ Tab created")

        print("4. Navigating to Twitter search...")
        await tab.get("https://x.com/search?q=bitcoin&f=live")
        print("   ✓ Navigated")

        print("5. Waiting for page load...")
        await asyncio.sleep(6)
        print("   ✓ Page loaded")

        print("6. Finding tweets...")
        tweets = await tab.find_elements("//article[@data-testid='tweet']")
        print(f"   ✓ Found {len(tweets)} tweet elements")

        print("\n7. Extracting tweet text...")
        collected_tweets = []
        for i, tweet in enumerate(tweets[:5]):
            try:
                text_elem = await tweet.find_element(".//div[@data-testid='tweetText']")
                text = await text_elem.text
                collected_tweets.append(text)
                print(f"   Tweet {i+1}: {text[:70]}...")
            except Exception as e:
                print(f"   Error on tweet {i+1}: {e}")

        print(f"\n✓ Successfully collected {len(collected_tweets)} tweets!")

        print("\n8. Closing browser...")
        await browser.close()
        print("   ✓ Browser closed")

        print("\n" + "=" * 60)
        print("SUCCESS! ZenDriver is working on Linux")
        print("=" * 60)

        return True

    except Exception as e:
        print(f"\n✗ ERROR: {e}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    print("Installing ZenDriver...")
    install_zendriver()

    print("\nRunning test...\n")
    success = asyncio.run(test_zendriver())

    sys.exit(0 if success else 1)
