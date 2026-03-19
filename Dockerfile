# Build stage
FROM rust:latest as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY models ./models
COPY sentiment_predictor.py .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM python:3.11-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    chromium-browser \
    chromium-driver \
    xvfb \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
RUN pip install --no-cache-dir scikit-learn numpy zendriver requests

# Copy binary from builder
COPY --from=builder /app/target/release/sentiment-api /app/sentiment-api

# Copy models and scripts
COPY models ./models
COPY sentiment_predictor.py .
COPY tweet_scraper.py .
COPY index.html .

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./sentiment-api"]
