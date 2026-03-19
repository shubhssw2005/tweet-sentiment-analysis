use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Deserialize, Serialize};
use chrono::Utc;

mod models;
mod twitter;
mod sentiment;

use models::SentimentModel;
use twitter::TwitterClient;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnalysisResult {
    pub id: String,
    pub text: String,
    pub sentiment: String,
    pub confidence: f32,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct StreamRequest {
    pub topics: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct StreamResponse {
    pub status: String,
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Initializing Sentiment Analysis API...");

    // Initialize Python sentiment model
    let model = Arc::new(
        SentimentModel::new()
            .expect("Failed to initialize sentiment model")
    );

    // Initialize Twitter client
    let twitter_client = Arc::new(TwitterClient::new());

    // Shared state for streaming results
    let results = Arc::new(Mutex::new(Vec::<AnalysisResult>::new()));

    log::info!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        let model = Arc::clone(&model);
        let twitter_client = Arc::clone(&twitter_client);
        let results = Arc::clone(&results);

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(model))
            .app_data(web::Data::new(twitter_client))
            .app_data(web::Data::new(results))
            .route("/", web::get().to(serve_index))
            .route("/health", web::get().to(health_check))
            .route("/analyze", web::post().to(analyze_sentiment))
            .route("/stream/start", web::post().to(start_stream))
            .route("/stream/stop", web::post().to(stop_stream))
            .route("/results", web::get().to(get_results))
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

async fn serve_index() -> HttpResponse {
    match std::fs::read_to_string("index.html") {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(_) => HttpResponse::NotFound().body("index.html not found")
    }
}

async fn analyze_sentiment(
    req: web::Json<AnalyzeRequest>,
    model: web::Data<Arc<SentimentModel>>,
) -> HttpResponse {
    log::info!("Analyzing sentiment for text: {}", &req.text[..req.text.len().min(50)]);

    match model.predict(&req.text) {
        Ok(result) => {
            log::info!("Prediction: {} (confidence: {})", result.sentiment, result.confidence);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            log::error!("Prediction error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

// Global state for streaming topics
static mut STREAMING_TOPICS: Vec<String> = Vec::new();

async fn start_stream(
    req: web::Json<StreamRequest>,
    model: web::Data<Arc<SentimentModel>>,
    twitter_client: web::Data<Arc<TwitterClient>>,
    results: web::Data<Arc<Mutex<Vec<AnalysisResult>>>>,
) -> HttpResponse {
    let topics = req.topics.clone();
    
    if topics.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Please provide at least one topic"
        }));
    }

    unsafe {
        if !STREAMING_TOPICS.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Stream already running with topics: {}",
                "topics": STREAMING_TOPICS.clone()
            }));
        }
        STREAMING_TOPICS = topics.clone();
    }

    let model_clone = Arc::clone(&model);
    let client_clone = Arc::clone(&twitter_client);
    let results_clone = Arc::clone(&results);
    let topics_clone = topics.clone();

    task::spawn(async move {
        log::info!("Starting tweet stream for topics: {:?}", topics_clone);
        
        loop {
            unsafe {
                if STREAMING_TOPICS.is_empty() {
                    log::info!("Stream stopped");
                    break;
                }
            }

            match client_clone.fetch_tweets_for_topics(&topics_clone).await {
                Ok(tweets) => {
                    for tweet in tweets {
                        match model_clone.predict(&tweet.text) {
                            Ok(mut result) => {
                                result.id = uuid::Uuid::new_v4().to_string();
                                result.timestamp = Utc::now().to_rfc3339();
                                
                                log::info!(
                                    "Tweet: {} | Sentiment: {} | Confidence: {:.2}%",
                                    &tweet.text[..tweet.text.len().min(50)],
                                    result.sentiment,
                                    result.confidence * 100.0
                                );

                                let mut res = results_clone.lock().unwrap();
                                res.push(result);
                                if res.len() > 100 {
                                    res.remove(0);
                                }
                            }
                            Err(e) => log::error!("Sentiment prediction error: {}", e),
                        }
                    }
                }
                Err(e) => log::error!("Error fetching tweets: {}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });

    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "topics": topics,
        "message": format!("Tweet stream started for topics: {}. Results available at /results", topics.join(", "))
    }))
}

async fn stop_stream(
    results: web::Data<Arc<Mutex<Vec<AnalysisResult>>>>,
) -> HttpResponse {
    unsafe {
        STREAMING_TOPICS.clear();
    }
    
    // Clear all results when stopping
    let mut res = results.lock().unwrap();
    res.clear();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "stopped",
        "message": "Tweet stream stopped and results cleared"
    }))
}

async fn get_results(
    results: web::Data<Arc<Mutex<Vec<AnalysisResult>>>>,
) -> HttpResponse {
    let res = results.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "count": res.len(),
        "results": res.clone()
    }))
}
