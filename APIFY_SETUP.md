# Apify Integration Guide

## What is Apify?

Apify is a web scraping platform that provides free APIs for scraping data from websites, including Twitter.

**Benefits:**

- ✅ No authentication required for basic usage
- ✅ Free tier available
- ✅ Easy to use
- ✅ Reliable and fast
- ✅ Automatic fallback to mock data if unavailable

## How It Works

The Rust backend calls Apify's Twitter Scraper API:

```
Rust Backend → Apify API → Twitter Data → Sentiment Analysis → Results
```

### API Endpoint Used

```
https://api.apify.com/v2/actor-tasks/apify~twitter-scraper/run-sync-get-dataset-items
```

### Query Parameters

- `searchTerms`: "sentiment OR emotion OR happy OR sad"
- `maxItems`: 10 tweets per request
- `timeout`: 30 seconds

## Features

1. **No Authentication**: Works without API keys
2. **Automatic Fallback**: Uses mock tweets if Apify is unavailable
3. **Concurrent Requests**: Handles multiple requests simultaneously
4. **Error Handling**: Graceful degradation with logging

## Usage

### Start Streaming Real Tweets

```bash
# Terminal 1: Start the server
cargo run --release

# Terminal 2: Start streaming
curl -X POST http://127.0.0.1:8080/stream/start

# Terminal 3: Get results
curl http://127.0.0.1:8080/results
```

### Response Example

```json
{
  "count": 5,
  "results": [
    {
      "id": "tweet_123",
      "text": "I love this product! Amazing quality!",
      "sentiment": "POSITIVE",
      "confidence": 0.95,
      "timestamp": "2024-03-19T10:30:00Z"
    },
    {
      "id": "tweet_124",
      "text": "Worst experience ever, very disappointed",
      "sentiment": "NEGATIVE",
      "confidence": 0.88,
      "timestamp": "2024-03-19T10:31:00Z"
    }
  ]
}
```

## Limitations

- **Rate Limiting**: Apify has rate limits on free tier
- **Latency**: ~2-5 seconds per request
- **Search Terms**: Limited to predefined queries
- **Tweet Count**: Max 10 per request

## Upgrading to Paid Apify

If you need more tweets/requests:

1. Go to https://apify.com/
2. Sign up for paid plan
3. Get API token
4. Update `src/twitter.rs` to use authentication

## Troubleshooting

### Getting mock tweets instead of real ones

**Reason**: Apify API is unavailable or rate limited

**Solution**:

- Check internet connection
- Wait a few minutes and retry
- Check Apify service status

### Slow responses

**Reason**: Apify API is slow or network latency

**Solution**:

- Increase timeout in `src/twitter.rs`
- Reduce `maxItems` to 5
- Adjust stream interval to 15 seconds

### No tweets returned

**Reason**: Search terms don't match any tweets

**Solution**:

- Update search terms in `src/twitter.rs`
- Try broader terms like "happy" or "sad"

## Code Location

The Apify integration is in `src/twitter.rs`:

```rust
pub async fn fetch_tweets(&self) -> Result<Vec<Tweet>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.apify.com/v2/actor-tasks/apify~twitter-scraper/run-sync-get-dataset-items")
        .query(&[
            ("searchTerms", "sentiment OR emotion OR happy OR sad"),
            ("maxItems", "10"),
            ("timeout", "30"),
        ])
        .send()
        .await;

    // ... error handling and mock fallback
}
```

## Next Steps

1. Build and run the project
2. Start streaming tweets
3. Monitor sentiment in real-time
4. Build a frontend to visualize results
5. Consider upgrading to paid Apify for production use

## Resources

- Apify Website: https://apify.com/
- Twitter Scraper: https://apify.com/apify/twitter-scraper
- Apify Docs: https://docs.apify.com/
