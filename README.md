# Tweet Sentiment Analysis

A real-time sentiment analysis system for tweets using machine learning. Built with Rust (Actix-web) backend and modern web frontend.

## Features

- **Real-time Tweet Streaming** - Fetch live tweets from Twitter241 API
- **Sentiment Analysis** - Classify tweets as positive or negative using trained ML models
- **Professional UI** - Clean, modern interface with real-time updates
- **RESTful API** - Easy-to-use endpoints for integration
- **Scalable** - Handles up to 100 concurrent tweets with efficient memory management

## Tech Stack

### Backend

- **Rust** with Actix-web framework
- **Python** for ML model inference (scikit-learn)
- **Tokio** for async runtime
- **Reqwest** for HTTP requests

### Frontend

- **HTML5** with modern CSS
- **Vanilla JavaScript** (no dependencies)
- **Responsive Design** - Works on desktop and mobile

### ML Models

- **TF-IDF Vectorizer** - Text feature extraction
- **Logistic Regression** - Sentiment classification

## Project Structure

```
.
├── src/
│   ├── main.rs           # API server and endpoints
│   ├── models.rs         # ML model inference
│   ├── twitter.rs        # Tweet fetching logic
│   └── sentiment.rs      # Sentiment analysis utilities
├── models/
│   ├── vectoriser-ngram-(1,2).pickle
│   ├── Sentiment-LR.pickle
│   └── Sentiment-BNB.pickle
├── index.html            # Frontend UI
├── sentiment_predictor.py # Python ML inference script
├── Cargo.toml            # Rust dependencies
└── README.md
```

## Prerequisites

- **Rust** 1.70+ ([Install](https://rustup.rs/))
- **Python** 3.8+ with scikit-learn
- **Twitter241 API Key** from RapidAPI ([Get here](https://rapidapi.com/))

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/tweet-sentiment-analysis.git
cd tweet-sentiment-analysis
```

### 2. Set Up Environment

```bash
# Set your RapidAPI key
export RAPIDAPI_KEY="your_api_key_here"

# Install Python dependencies
pip install scikit-learn numpy

# Build Rust backend
cargo build --release
```

### 3. Run Locally

```bash
# Start the server
./target/release/sentiment-api

# Open in browser
open http://127.0.0.1:8080
```

## API Endpoints

### Health Check

```bash
GET /health
```

### Analyze Single Tweet

```bash
POST /analyze
Content-Type: application/json

{
  "text": "I love this product!"
}
```

### Start Stream

```bash
POST /stream/start
Content-Type: application/json

{
  "topics": ["bitcoin", "crypto", "tesla"]
}
```

### Stop Stream

```bash
POST /stream/stop
```

### Get Results

```bash
GET /results
```

Returns:

```json
{
  "count": 5,
  "results": [
    {
      "id": "uuid",
      "text": "tweet text",
      "sentiment": "POSITIVE",
      "confidence": 0.95,
      "timestamp": "2026-03-19T10:00:00+00:00"
    }
  ]
}
```

## Configuration

### Environment Variables

```bash
RAPIDAPI_KEY=your_api_key_here
RUST_LOG=info
```

### API Configuration

Edit `src/twitter.rs` to change:

- User IDs for tweet fetching
- API endpoint
- Request timeout

## Deployment

### Option 1: Deploy Backend on Render

1. Push code to GitHub
2. Go to [Render.com](https://render.com)
3. Create new Web Service
4. Connect GitHub repository
5. Set environment variables:
   - `RAPIDAPI_KEY`: Your API key
   - `RUST_LOG`: info
6. Build command: `cargo build --release`
7. Start command: `./target/release/sentiment-api`

### Option 2: Deploy Backend on Railway

1. Go to [Railway.app](https://railway.app)
2. Create new project
3. Connect GitHub repository
4. Set environment variables
5. Railway auto-detects Rust and deploys

### Option 3: Deploy Frontend on Netlify

1. Build frontend (already static HTML)
2. Go to [Netlify.com](https://netlify.com)
3. Drag and drop `index.html` or connect GitHub
4. Update API URL in frontend to point to deployed backend

### Full Stack Deployment (Recommended)

**Backend on Render:**

```
https://your-app.onrender.com
```

**Frontend on Netlify:**

- Update `API_URL` in `index.html` to your Render backend URL
- Deploy to Netlify

## Development

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### View Logs

```bash
tail -f /tmp/api.log
```

## Performance

- **Max Tweets**: 100 (auto-managed)
- **Fetch Interval**: 10 seconds
- **Response Time**: < 100ms
- **Memory Usage**: ~50MB

## Troubleshooting

### No tweets appearing

- Check API key is valid
- Verify internet connection
- Check logs: `tail -f /tmp/api.log`

### Sentiment predictions incorrect

- Models are trained on specific dataset
- Consider retraining with your data
- Check Python environment has scikit-learn

### Frontend not updating

- Clear browser cache (Ctrl+Shift+Delete)
- Check API URL is correct
- Verify backend is running

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## License

MIT License - see LICENSE file for details

## Support

For issues and questions:

- Open GitHub Issues
- Check existing documentation
- Review logs for errors

## Roadmap

- [ ] Add more ML models
- [ ] Support multiple languages
- [ ] Add user authentication
- [ ] Create mobile app
- [ ] Add data export features
- [ ] Implement caching layer

## Author

Created with ❤️ for real-time sentiment analysis

---

**Live Demo**: [Coming Soon]

**GitHub**: [Your Repository URL]
