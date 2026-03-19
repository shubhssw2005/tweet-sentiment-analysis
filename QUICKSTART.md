# Quick Start Guide

Get the Tweet Sentiment Analysis app running in 5 minutes.

## Local Development

### 1. Prerequisites

```bash
# Check Rust
rustc --version

# Check Python
python3 --version

# Install dependencies
pip install scikit-learn numpy
```

### 2. Clone & Setup

```bash
git clone https://github.com/yourusername/tweet-sentiment-analysis.git
cd tweet-sentiment-analysis

# Set API key
export RAPIDAPI_KEY="your_api_key_here"
```

### 3. Build & Run

```bash
# Build
cargo build --release

# Run
./target/release/sentiment-api

# Open browser
open http://127.0.0.1:8080
```

### 4. Use the App

1. Enter topics: `bitcoin, crypto, tesla`
2. Click "Start Stream"
3. Wait 10 seconds for tweets
4. See sentiment analysis in real-time

## Docker Setup

### 1. Build Image

```bash
docker build -t sentiment-api .
```

### 2. Run Container

```bash
docker run -p 8080:8080 \
  -e RAPIDAPI_KEY="your_api_key_here" \
  sentiment-api
```

### 3. Open Browser

```
http://localhost:8080
```

## Docker Compose

### 1. Start Services

```bash
docker-compose up
```

### 2. Open Browser

```
http://localhost:8080
```

### 3. Stop Services

```bash
docker-compose down
```

## Production Deployment

See [DEPLOYMENT.md](DEPLOYMENT.md) for full guide.

### Quick Deploy to Render

1. Push to GitHub
2. Go to [render.com](https://render.com)
3. Connect repository
4. Set `RAPIDAPI_KEY` environment variable
5. Deploy

### Quick Deploy to Netlify

1. Go to [netlify.com](https://netlify.com)
2. Drag and drop `index.html`
3. Get instant URL

## API Usage

### Analyze Single Tweet

```bash
curl -X POST http://127.0.0.1:8080/analyze \
  -H "Content-Type: application/json" \
  -d '{"text": "I love this!"}'
```

### Start Stream

```bash
curl -X POST http://127.0.0.1:8080/stream/start \
  -H "Content-Type: application/json" \
  -d '{"topics": ["bitcoin", "crypto"]}'
```

### Get Results

```bash
curl http://127.0.0.1:8080/results
```

### Stop Stream

```bash
curl -X POST http://127.0.0.1:8080/stream/stop
```

## Troubleshooting

### Port 8080 already in use

```bash
# Find process
lsof -i :8080

# Kill process
kill -9 <PID>
```

### Python not found

```bash
# Use python3 explicitly
which python3
```

### API key not working

1. Check key is correct
2. Verify RapidAPI subscription
3. Check internet connection

### No tweets appearing

1. Wait 10+ seconds
2. Check API key
3. Try different topics
4. Check logs: `tail -f /tmp/api.log`

## Next Steps

- Read [README.md](README.md) for full documentation
- Check [DEPLOYMENT.md](DEPLOYMENT.md) for production setup
- Explore API endpoints
- Customize frontend
- Add more features

## Support

- GitHub Issues: Report bugs
- Discussions: Ask questions
- Documentation: Check README.md

## Tips

- Use common topics for better results
- Monitor API rate limits
- Check logs for errors
- Test locally before deploying
- Keep API key secret

Enjoy! 🚀
