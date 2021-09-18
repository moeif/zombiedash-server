use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn init_conn() -> Option<Database> {
    dotenv().ok();
    let mongodb_url = env::var("MONGODB_URL").expect("MONGODB_URL missing");
    let dbname = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME missing");
    if let Ok(mut client_option) = ClientOptions::parse(mongodb_url).await {
        client_option.min_pool_size = Some(10);
        if let Ok(client) = Client::with_options(client_option) {
            let database = client.database(&dbname);
            return Some(database);
        };
    };

    return None;
}
