# Tweet Sentiment Analysis

A real-time sentiment analysis system for tweets using machine learning. Built with Rust (Actix-web) backend and modern web frontend.

## Features

- **Real-time Tweet Streaming** - Fetch live tweets from Twitter using GetXAPI
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
- **GetXAPI Key** from [GetXAPI](https://getxapi.com/) (free $0.1 credits, no card required)

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/shubhssw2005/tweet-sentiment-analysis.git
cd tweet-sentiment-analysis
```

### 2. Set Up Environment

```bash
# Set your GetXAPI key
export GETXAPI_KEY="your_getxapi_key_here"

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
GETXAPI_KEY=your_getxapi_key_here
RUST_LOG=info
```

## Deployment

### Backend on Render

1. Push code to GitHub
2. Go to [Render.com](https://render.com)
3. Create new Web Service
4. Connect GitHub repository
5. Set environment variables:
   - `GETXAPI_KEY`: Your GetXAPI key
   - `RUST_LOG`: info
6. Build command: `cargo build --release`
7. Start command: `./target/release/sentiment-api`

### Frontend on Netlify

1. Go to [Netlify.com](https://netlify.com)
2. Connect GitHub repository
3. Update API URL in frontend to point to deployed backend
4. Deploy

## License

MIT License - see LICENSE file for details

## Live Demo

**Website**: [https://tweet-sentiment-26.netlify.app](https://tweet-sentiment-26.netlify.app)

**GitHub**: [https://github.com/shubhssw2005/tweet-sentiment-analysis](https://github.com/shubhssw2005/tweet-sentiment-analysis)
