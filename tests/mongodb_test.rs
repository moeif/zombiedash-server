use mongodb::Database;
use rocket_tut::data::mongo_connection;

#[rocket::async_test]
async fn mongodb_init_test() {
    let option_db = mongo_connection::init_conn().await;
    if let Some(db) = option_db {
        assert_eq!(1, 1);
    } else {
        assert_eq!(1, 2);
    }
}
