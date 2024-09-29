use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct RateLimitConfig {
    pub duration: Duration,
    pub max_requests: u32,
}

impl RateLimitConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let duration = env::var("RATE_LIMIT_DURATION")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);
        let max_requests = env::var("RATE_LIMIT_MAX_REQUESTS")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100);

        Self {
            duration: Duration::from_secs(duration),
            max_requests,
        }
    }
}
