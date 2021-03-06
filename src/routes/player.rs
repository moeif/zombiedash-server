use super::common::ApiResponse;
use crate::data::connector::Connector;
use crate::data::model;
use mongodb::bson::doc;
use redis::{self, Connection};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;

const MONGODB_COLLECTION: &str = "zombiedash_players";
const REDIS_COLLECTION: &str = "zombiedash_players";

fn set_player_to_redis(player_id: String, name: String, redis_client: &redis::Client) {
    if let Ok(mut redis_connection) = redis_client.get_connection() {
        if let Ok(_) = redis::cmd("HSET")
            .arg(REDIS_COLLECTION)
            .arg(player_id.clone())
            .arg(name.clone())
            .query::<u32>(&mut redis_connection)
        {
            info!("用户数据存入Redis, playerid: {}, name: {}", player_id, name);
        }
    } else {
        info!("用户数据存入Redis失败，获取Redis连接失败!");
    }
}

pub fn get_player_name_from_redis(connection: &mut Connection, player_id: &str) -> Option<String> {
    if let Ok(result) = redis::cmd("HGET")
        .arg(REDIS_COLLECTION)
        .arg(player_id.clone())
        .query::<String>(connection)
    {
        return Some(result);
    }
    return None;
}

#[get("/sumplayers")]
pub async fn sum_players_rt(connector: &State<Connector>) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        if let Ok(result) = redis::cmd("HLEN")
            .arg(REDIS_COLLECTION)
            .query::<u32>(&mut redis_connection)
        {
            info!("从Redis中获取数据成功，当前用户数量: {}", result);
            return ApiResponse::ok(json!([result]));
        }
    };

    let player_coll = connector
        .mongodb
        .collection::<model::Player>(MONGODB_COLLECTION);
    if let Ok(count) = player_coll.count_documents(None, None).await {
        ApiResponse::ok(json!([count]))
    } else {
        ApiResponse::internal_err()
    }
}

#[post("/player", format = "json", data = "<player>")]
pub async fn new_player_rt(
    connector: &State<Connector>,
    player: Json<model::ClientablePlayer>,
) -> ApiResponse {
    // 先判断Redis是否有这个用户，如果没有，再看数据库
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        if let Ok(result) = redis::cmd("HEXISTS")
            .arg(REDIS_COLLECTION)
            .arg(&player.player_id)
            .query::<u32>(&mut redis_connection)
        {
            if result > 0 {
                info!(
                    "从Redis中获取用户数据成功，当前用户已存在, player_id: {}",
                    &player.player_id
                );
                return ApiResponse::empty_ok();
            }
        }
    };

    set_player_to_redis(
        player.player_id.clone(),
        player.name.clone(),
        &connector.redis,
    );

    let player_coll = connector
        .mongodb
        .collection::<model::Player>(MONGODB_COLLECTION);
    if let Ok(find_one) = player_coll
        .find_one(Some(doc! {"player_id": player.player_id.clone()}), None)
        .await
    {
        if let Some(_) = find_one {
            return ApiResponse::empty_ok();
        } else {
            let new_player = model::Player::from_clientable(player.clone());
            if let Ok(_) = player_coll.insert_one(new_player, None).await {
                return ApiResponse::empty_ok();
            }
        }
    } else {
        info!("从Mongodb中获查找用户 {} 失败!", player.player_id);
    }

    return ApiResponse::internal_err();
}

#[get("/player/<player_id>")]
pub async fn info_player_rt(connector: &State<Connector>, player_id: String) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        if let Ok(result) = redis::cmd("HGET")
            .arg(REDIS_COLLECTION)
            .arg(player_id.clone())
            .query::<String>(&mut redis_connection)
        {
            let clientable_player = model::ClientablePlayer::new(player_id.clone(), result);
            info!("从Redis中查询用户信息成功, player_id: {}", player_id);
            return ApiResponse::ok(json!(clientable_player));
        }
    }

    let player_coll = connector
        .mongodb
        .collection::<model::Player>(MONGODB_COLLECTION);
    if let Ok(find_one) = player_coll
        .find_one(Some(doc! {"player_id": player_id}), None)
        .await
    {
        if let Some(found_player) = find_one {
            let clientable_player = model::ClientablePlayer::from_player(found_player);

            set_player_to_redis(
                clientable_player.player_id.clone(),
                clientable_player.name.clone(),
                &connector.redis,
            );

            return ApiResponse::ok(json!(clientable_player));
        }
    }

    return ApiResponse::internal_err();
}
