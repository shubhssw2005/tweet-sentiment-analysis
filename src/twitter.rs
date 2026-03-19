use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterUser {
    #[serde(default)]
    id: String,  // Changed from u64 to String since API returns string
    #[serde(default)]
    screen_name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    status: Option<TwitterStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterStatus {
    #[serde(default)]
    text: String,
    #[serde(default)]
    created_at: String,
}

pub struct TwitterClient {
    api_key: String,
}

impl TwitterClient {
    pub fn new() -> Self {
        let api_key = std::env::var("RAPIDAPI_KEY")
            .unwrap_or_else(|_| "16abad933dmshcb92fdf62a236e0p124fe0jsn6f43f76410ef".to_string());
        
        log::info!("✅ Twitter241 API Client initialized");
        TwitterClient { api_key }
    }

    pub async fn fetch_tweets_for_topics(&self, topics: &[String]) -> Result<Vec<Tweet>, Box<dyn Error>> {
        // Try Twitter241 API first
        match self.fetch_from_twitter241(topics).await {
            Ok(tweets) if !tweets.is_empty() => return Ok(tweets),
            _ => {}
        }
        
        // Fallback to twitter-api45
        self.fetch_from_twitter_api45(topics).await
    }

    async fn fetch_from_twitter241(&self, topics: &[String]) -> Result<Vec<Tweet>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        
        let search_query = topics.join(" ");
        log::info!("🔍 Fetching tweets from Twitter241 API for topics: {}", search_query);
        
        let user_ids = "1222790936679206913,133938408,34186021";
        
        let response = client
            .get("https://twitter241.p.rapidapi.com/get-users-v2")
            .header("x-rapidapi-key", &self.api_key)
            .header("x-rapidapi-host", "twitter241.p.rapidapi.com")
            .header("Content-Type", "application/json")
            .query(&[("users", user_ids)])
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Twitter241 API error {}: {}", status, body);
            return Err(format!("Twitter241 API error {}: {}", status, body).into());
        }

        let data: serde_json::Value = response.json().await?;
        
        let mut tweets = Vec::new();
        
        let users_data = if data.is_array() {
            data.as_array().unwrap().clone()
        } else if let Some(result) = data.get("result") {
            if result.is_array() {
                result.as_array().unwrap().clone()
            } else {
                vec![result.clone()]
            }
        } else if let Some(data_field) = data.get("data") {
            if data_field.is_array() {
                data_field.as_array().unwrap().clone()
            } else {
                vec![data_field.clone()]
            }
        } else if let Some(users_field) = data.get("users") {
            if users_field.is_array() {
                users_field.as_array().unwrap().clone()
            } else {
                vec![users_field.clone()]
            }
        } else {
            vec![data]
        };
        
        for user_val in users_data {
            if let Ok(user) = serde_json::from_value::<TwitterUser>(user_val.clone()) {
                if let Some(status) = user.status {
                    if !status.text.is_empty() {
                        tweets.push(Tweet {
                            id: format!("{}_{}", user.id, user.screen_name),
                            text: status.text.clone(),
                            created_at: status.created_at,
                        });
                    }
                }
            }
        }
        
        if tweets.is_empty() {
            log::warn!("No tweets found from Twitter241");
            return Err("No tweets found".into());
        }
        
        log::info!("✅ Fetched {} tweets from Twitter241 API", tweets.len());
        Ok(tweets)
    }

    async fn fetch_from_twitter_api45(&self, topics: &[String]) -> Result<Vec<Tweet>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        
        let search_query = topics.join(" ");
        log::info!("🔍 Fetching tweets from twitter-api45 for topics: {}", search_query);
        
        let response = client
            .get("https://twitter-api45.p.rapidapi.com/community_info.php")
            .header("x-rapidapi-key", &self.api_key)
            .header("x-rapidapi-host", "twitter-api45.p.rapidapi.com")
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("twitter-api45 error {}: {}", status, body);
            return Err(format!("twitter-api45 error {}: {}", status, body).into());
        }

        let data: serde_json::Value = response.json().await?;
        
        let mut tweets = Vec::new();
        
        // Parse response and extract tweets
        if let Some(items) = data.get("data").and_then(|v| v.as_array()) {
            for item in items {
                if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                    if !text.is_empty() {
                        let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
                        let created_at = item.get("created_at").and_then(|v| v.as_str()).unwrap_or("");
                        
                        tweets.push(Tweet {
                            id: id.to_string(),
                            text: text.to_string(),
                            created_at: created_at.to_string(),
                        });
                    }
                }
            }
        }
        
        if tweets.is_empty() {
            log::warn!("No tweets found from twitter-api45");
            return Err("No tweets found".into());
        }
        
        log::info!("✅ Fetched {} tweets from twitter-api45", tweets.len());
        Ok(tweets)
    }
}
