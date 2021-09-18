use redis;
use std::env;

pub async fn init_conn() -> Option<redis::Client> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL missing");
    if let Ok(client) = redis::Client::open(redis_url) {
        return Some(client);
    }

    return None;
}
