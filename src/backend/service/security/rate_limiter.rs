use ratelimit::*;

let rate_limiter = RateLimiter::builder()
    .capacity(10)
    .quantum(1)
    .interval(Duration::from_secs(1))
    .build()
    .unwrap();
