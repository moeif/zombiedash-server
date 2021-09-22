use super::common::ApiResponse;
use crate::data::connector::Connector;
use crate::data::model;
use mongodb::bson::doc;
use redis;
use redis::Connection;
use rocket::serde::json::serde_json::{self, json};
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;

const REDIS_COLLECTION: &str = "zombiedash_worship";
const REDIS_WORSHIP_NAMES_COLLECTION: &str = "zombiedash_worship_names";

fn add_worship(connection: &mut Connection, player_id: &str) {
    let field_name = format!("{}", player_id);
    let prev_worship = if let Ok(result) = redis::cmd("HGET")
        .arg(REDIS_COLLECTION)
        .arg(field_name.clone())
        .query::<u32>(connection)
    {
        result
    } else {
        0
    };

    let worship = prev_worship + 1;

    if let Ok(_) = redis::cmd("HSET")
        .arg(REDIS_COLLECTION)
        .arg(field_name.clone())
        .arg(worship)
        .query::<u32>(connection)
    {
        println!("膜拜新增成功！")
    } else {
        println!("膜拜新增失败！");
    }
}

fn get_worship(connection: &mut Connection, player_id: &str) -> u32 {
    let field_name = format!("{}", player_id);
    if let Ok(result) = redis::cmd("HGET")
        .arg(REDIS_COLLECTION)
        .arg(field_name)
        .query::<u32>(connection)
    {
        return result;
    }

    return 0;
}

fn add_worship_name(connection: &mut Connection, player_id: &str, src_player_id: &str) {
    if let Some(player_name) = super::get_player_name_from_redis(connection, src_player_id) {
        let field_name = format!("{}", player_id);
        let json_str = if let Ok(result) = redis::cmd("HGET")
            .arg(REDIS_WORSHIP_NAMES_COLLECTION)
            .arg(field_name.clone())
            .query::<String>(connection)
        {
            result
        } else {
            String::new()
        };

        let mut names: Vec<String> = if json_str.len() > 0 {
            if let Ok(result) = serde_json::from_str::<Vec<String>>(&json_str) {
                result
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        names.push(player_name);

        if let Ok(json_str) = serde_json::to_string(&names) {
            // 将新的JsonStr存入Redis
            if let Ok(_) = redis::cmd("HSET")
                .arg(REDIS_WORSHIP_NAMES_COLLECTION)
                .arg(field_name)
                .arg(json_str)
                .query::<u32>(connection)
            {
                println!("添加新的膜拜名单成功!");
            }
        } else {
            println!("添加新的膜拜姓名序列化失败！");
        }
    }
}

// 一旦调用，就会返回数据，然后删除数据
fn get_worship_names(connection: &mut Connection, player_id: &str) -> Vec<String> {
    let field_name = format!("{}", player_id);
    let json_str = if let Ok(result) = redis::cmd("HGET")
        .arg(REDIS_WORSHIP_NAMES_COLLECTION)
        .arg(field_name.clone())
        .query::<String>(connection)
    {
        result
    } else {
        String::new()
    };

    let names: Vec<String> = if json_str.len() > 0 {
        if let Ok(result) = serde_json::from_str::<Vec<String>>(&json_str) {
            result
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    if let Ok(_) = redis::cmd("HDEL")
        .arg(REDIS_WORSHIP_NAMES_COLLECTION)
        .arg(field_name.clone())
        .query::<u32>(connection)
    {
        println!("膜拜名单删除成功");
    }

    return names;
}

#[get("/worship/<player_id>")]
pub async fn get_worship_rt(connector: &State<Connector>, player_id: String) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        let worship_num = get_worship(&mut redis_connection, &player_id);
        return ApiResponse::ok(json!({ "worship": worship_num }));
    } else {
        println!("Get Worship Error!");
    }

    return ApiResponse::internal_err();
}

#[post("/worship", format = "json", data = "<worshipinfo>")]
pub async fn post_worship_rt(
    connector: &State<Connector>,
    worshipinfo: Json<model::ClientableWorship>,
) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        add_worship(&mut redis_connection, &worshipinfo.dst_player_id);
        add_worship_name(
            &mut redis_connection,
            &worshipinfo.dst_player_id,
            &worshipinfo.src_player_id,
        );
        return ApiResponse::empty_ok();
    }
    return ApiResponse::internal_err();
}

#[get("/worshipnames/<player_id>")]
pub async fn get_worship_names_rt(connector: &State<Connector>, player_id: String) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        let worship_names_vec = get_worship_names(&mut redis_connection, &player_id);
        return ApiResponse::ok(json!(worship_names_vec));
    }
    return ApiResponse::internal_err();
}
