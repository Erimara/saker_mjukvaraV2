
use ratelimit::Ratelimiter;
use std::time::Duration;

pub fn limit_requests(){
    Ratelimiter::builder(1000, Duration::from_secs(3600))
        .max_tokens(1000)
        .initial_available(1000)
        .build()
        .unwrap();
}
pub fn is_request_allowed(rate_limiter: Ratelimiter) -> Result<(), Duration>{
    for _ in 0..10 {
        if let Err(sleep) = rate_limiter.try_wait() {
            std::thread::sleep(sleep);
            continue;
        }
        return Ok(());
    }
    Err(Duration::from_secs(60))
}
