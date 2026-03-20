use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Deserialize)]
struct GetXAPIResponse {
    tweets: Vec<GetXAPITweet>,
}

#[derive(Deserialize)]
struct GetXAPITweet {
    id: String,
    text: String,
    #[serde(default)]
    created_at: String,
}

pub struct TwitterClient {
    api_key: String,
    client: reqwest::Client,
}

impl TwitterClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let api_key = std::env::var("GETXAPI_KEY")
            .unwrap_or_else(|_| "demo".to_string());
        
        log::info!("✅ GetXAPI Twitter Client initialized");
        
        Ok(TwitterClient {
            api_key,
            client: reqwest::Client::new(),
        })
    }

    pub async fn fetch_tweets_for_topics(&self, topics: &[String]) -> Result<Vec<Tweet>, Box<dyn Error>> {
        let query = topics.join(" OR ");
        
        log::info!("🔍 Fetching REAL tweets from GetXAPI for topics: {}", query);
        
        let url = format!(
            "https://api.getxapi.com/twitter/tweet/advanced_search?q={}&product=Latest",
            urlencoding::encode(&query)
        );
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = format!("GetXAPI error: {}", response.status());
            log::error!("{}", error);
            return Err(error.into());
        }
        
        let data: GetXAPIResponse = response.json().await?;
        
        let tweets: Vec<Tweet> = data.tweets
            .into_iter()
            .filter(|t| !t.text.is_empty())
            .map(|t| Tweet {
                id: t.id,
                text: t.text,
                created_at: if t.created_at.is_empty() {
                    chrono::Utc::now().to_rfc3339()
                } else {
                    t.created_at
                },
            })
            .collect();
        
        if tweets.is_empty() {
            log::warn!("⚠️ No tweets found from GetXAPI");
            return Err("No tweets found".into());
        }
        
        log::info!("✅ Fetched {} REAL tweets from GetXAPI", tweets.len());
        Ok(tweets)
    }
}
