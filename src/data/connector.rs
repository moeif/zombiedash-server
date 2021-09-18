use super::mongo_connection;
use super::redis_connection;
use mongodb::Database;
use redis::Client;

pub struct Connector {
    pub redis: Client,
    pub mongodb: Database,
}

pub async fn init_conn() -> Option<Connector> {
    if let Some(mongodb) = mongo_connection::init_conn().await {
        if let Some(redis) = redis_connection::init_conn().await {
            return Some(Connector { redis, mongodb });
        }
    }

    return None;
}
