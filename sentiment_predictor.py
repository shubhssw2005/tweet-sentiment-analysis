#!/usr/bin/env python3
"""
Sentiment prediction helper for Rust backend
Loads pickle models and performs predictions
"""

import sys
import json
import pickle
import os


class SentimentPredictor:
    def __init__(self, model_dir="models"):
        self.model_dir = model_dir
        self.vectorizer = None
        self.model = None
        self.encoder = None
        self.load_models()

    def load_models(self):
        """Load all pickle models"""
        try:
            # Load vectorizer
            vectorizer_path = os.path.join(
                self.model_dir, "vectoriser-ngram-(1,2).pickle"
            )
            with open(vectorizer_path, "rb") as f:
                self.vectorizer = pickle.load(f)
            print(f"Loaded vectorizer from {vectorizer_path}", file=sys.stderr)

            # Load model
            model_path = os.path.join(self.model_dir, "model.pkl")
            with open(model_path, "rb") as f:
                self.model = pickle.load(f)
            print(f"Loaded model from {model_path}", file=sys.stderr)

            # Load encoder
            encoder_path = os.path.join(self.model_dir, "encoder.pkl")
            with open(encoder_path, "rb") as f:
                self.encoder = pickle.load(f)
            print(f"Loaded encoder from {encoder_path}", file=sys.stderr)

        except Exception as e:
            print(f"Error loading models: {e}", file=sys.stderr)
            raise

    def predict(self, text):
        """Predict sentiment for given text"""
        try:
            # Vectorize text
            X = self.vectorizer.transform([text])

            # Get prediction
            prediction = self.model.predict(X)[0]
            probabilities = self.model.predict_proba(X)[0]
            confidence = float(probabilities.max())

            # Decode label
            label = self.encoder.inverse_transform([prediction])[0]

            return {"text": text, "sentiment": label, "confidence": confidence}
        except Exception as e:
            print(f"Error during prediction: {e}", file=sys.stderr)
            raise


def main():
    """Main entry point for command-line usage"""
    if len(sys.argv) < 2:
        print("Usage: python sentiment_predictor.py '<text>'", file=sys.stderr)
        sys.exit(1)

    text = sys.argv[1]

    try:
        predictor = SentimentPredictor()
        result = predictor.predict(text)
        print(json.dumps(result))
    except Exception as e:
        print(json.dumps({"error": str(e)}), file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
