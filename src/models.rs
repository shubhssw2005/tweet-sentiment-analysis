use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use crate::AnalysisResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentimentResult {
    pub text: String,
    pub sentiment: String,
    pub confidence: f32,
}

pub struct SentimentModel {
    python_path: String,
}

impl SentimentModel {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Verify Python is available
        let output = Command::new("python3")
            .arg("--version")
            .output()?;

        if !output.status.success() {
            return Err("Python3 not found".into());
        }

        log::info!("Python3 found: {:?}", String::from_utf8_lossy(&output.stdout));

        Ok(SentimentModel {
            python_path: "python3".to_string(),
        })
    }

    pub fn predict(&self, text: &str) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
        // Create Python script to run prediction
        let python_code = format!(
            r#"
import sys
import json
import pickle

try:
    # Load models
    with open('models/vectoriser-ngram-(1,2).pickle', 'rb') as f:
        vectorizer = pickle.load(f)
    
    with open('models/Sentiment-LR.pickle', 'rb') as f:
        model = pickle.load(f)
    
    # Predict
    text = {}
    X = vectorizer.transform([text])
    prediction = model.predict(X)[0]
    probability = model.predict_proba(X)[0].max()
    
    # Map prediction to label (0=NEGATIVE, 1=POSITIVE)
    label = "POSITIVE" if prediction == 1 else "NEGATIVE"
    
    result = {{
        "text": text,
        "sentiment": label,
        "confidence": float(probability)
    }}
    
    print(json.dumps(result))
except Exception as e:
    print(json.dumps({{"error": str(e)}}), file=sys.stderr)
    sys.exit(1)
"#,
            serde_json::to_string(&text)?
        );

        // Execute Python
        let child = Command::new(&self.python_path)
            .arg("-c")
            .arg(&python_code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Python error: {}", error).into());
        }

        let result_str = String::from_utf8(output.stdout)?;
        let result: SentimentResult = serde_json::from_str(&result_str)?;

        Ok(AnalysisResult {
            id: uuid::Uuid::new_v4().to_string(),
            text: result.text,
            sentiment: result.sentiment,
            confidence: result.confidence,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}
