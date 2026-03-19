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
        let client = reqwest::Client::new();
        
        let search_query = topics.join(" ");
        log::info!("🔍 Fetching REAL tweets from Twitter241 API for topics: {}", search_query);
        
        // Use the get-users-v2 endpoint with user IDs
        // For now, using popular user IDs that tweet about crypto/finance
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
            log::error!("❌ Twitter241 API error {}: {}", status, body);
            return Err(format!("Twitter241 API error {}: {}", status, body).into());
        }

        let data: serde_json::Value = response.json().await?;
        
        let mut tweets = Vec::new();
        
        // Parse users from response - API returns object with various possible structures
        let users_data = if data.is_array() {
            // Direct array response
            data.as_array().unwrap().clone()
        } else if let Some(result) = data.get("result") {
            // Response has "result" key
            if result.is_array() {
                result.as_array().unwrap().clone()
            } else {
                vec![result.clone()]
            }
        } else if let Some(data_field) = data.get("data") {
            // Response has "data" key
            if data_field.is_array() {
                data_field.as_array().unwrap().clone()
            } else {
                vec![data_field.clone()]
            }
        } else if let Some(users_field) = data.get("users") {
            // Response has "users" key
            if users_field.is_array() {
                users_field.as_array().unwrap().clone()
            } else {
                vec![users_field.clone()]
            }
        } else {
            // Treat entire response as single user object
            vec![data]
        };
        
        // Extract tweets from user statuses
        for user_val in users_data {
            if let Ok(user) = serde_json::from_value::<TwitterUser>(user_val.clone()) {
                if let Some(status) = user.status {
                    if !status.text.is_empty() {
                        // Filter by topics - only include if tweet contains any topic keyword
                        let text_lower = status.text.to_lowercase();
                        let matches_topic = topics.iter().any(|topic| {
                            text_lower.contains(&topic.to_lowercase())
                        });
                        
                        if matches_topic {
                            tweets.push(Tweet {
                                id: format!("{}_{}", user.id, user.screen_name),
                                text: status.text.clone(),
                                created_at: status.created_at,
                            });
                        }
                    }
                }
            }
        }
        
        if tweets.is_empty() {
            log::warn!("⚠️ No tweets found matching topics: {:?}", topics);
            return Err("No tweets found for these topics".into());
        }
        
        log::info!("✅ Fetched {} REAL tweets from Twitter241 API matching topics", tweets.len());
        Ok(tweets)
    }
}
