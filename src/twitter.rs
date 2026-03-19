use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

pub struct TwitterClient;

impl TwitterClient {
    pub fn new() -> Self {
        log::info!("✅ ZenDriver Tweet Scraper initialized (Linux)");
        TwitterClient
    }

    pub async fn fetch_tweets_for_topics(&self, topics: &[String]) -> Result<Vec<Tweet>, Box<dyn Error>> {
        let topics_str = topics.join(",");
        
        log::info!("🔍 Fetching REAL tweets using ZenDriver for topics: {}", topics_str);
        
        // Call Python scraper with concurrent threads
        let num_threads = (topics.len() * 2).min(8);
        
        let output = std::process::Command::new("python3")
            .arg("tweet_scraper.py")
            .arg(&topics_str)
            .arg(num_threads.to_string())
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            log::error!("ZenDriver scraper error: {}", error);
            return Err(format!("Scraper error: {}", error).into());
        }
        
        let result_str = String::from_utf8(output.stdout)?;
        let data: serde_json::Value = serde_json::from_str(&result_str)?;
        
        let mut tweets = Vec::new();
        
        if let Some(items) = data.as_array() {
            for item in items {
                if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                    if !text.is_empty() {
                        tweets.push(Tweet {
                            id: uuid::Uuid::new_v4().to_string(),
                            text: text.to_string(),
                            created_at: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                }
            }
        }
        
        if tweets.is_empty() {
            log::warn!("⚠️ No tweets found from ZenDriver scraper");
            return Err("No tweets found".into());
        }
        
        log::info!("✅ Fetched {} REAL tweets using ZenDriver with concurrent threads", tweets.len());
        Ok(tweets)
    }
}
