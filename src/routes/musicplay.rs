use super::common::ApiResponse;
use crate::data::connector::Connector;
use crate::data::model;
use mongodb::bson::doc;
use redis;
use redis::Connection;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::State;
use rocket::*;

const REDIS_COLLECTION_NUM_OF_BATTLES: &str = "zombiedash_num_of_battles";

fn increase_value_of_set(connection: &mut Connection, collection_name: &str, field_name: &str) {
    let prev_value = if let Ok(result) = redis::cmd("HGET")
        .arg(collection_name)
        .arg(field_name)
        .query::<u32>(connection)
    {
        result
    } else {
        0
    };

    let new_value = prev_value + 1;
    if let Ok(_) = redis::cmd("HSET")
        .arg(collection_name)
        .arg(field_name)
        .arg(new_value)
        .query::<u32>(connection)
    {
        println!(
            "值刷新成功: {} - {}, {} -> {}",
            collection_name, field_name, prev_value, new_value
        );
    } else {
        println!(
            "ERROR, 值刷新失败: {} - {}, {} -> {}",
            collection_name, field_name, prev_value, new_value
        );
    }
}

fn add_number_of_battles(connection: &mut Connection, player_id: &str, music_id: u32, diff: u32) {
    // 添加总战斗次数
    increase_value_of_set(connection, REDIS_COLLECTION_NUM_OF_BATTLES, player_id);

    // 添加当前歌曲及难度的战斗次数
    let field_name = format!("{}_{}_{}", player_id, music_id, diff);
    increase_value_of_set(connection, REDIS_COLLECTION_NUM_OF_BATTLES, &field_name);
}

fn get_num_of_battles(
    connection: &mut Connection,
    player_id: &str,
    music_id: u32,
    diff: u32,
) -> u32 {
    let field_name = if music_id > 0 {
        format!("{}_{}_{}", player_id, music_id, diff)
    } else {
        format!("{}", player_id)
    };
    if let Ok(result) = redis::cmd("HGET")
        .arg(REDIS_COLLECTION_NUM_OF_BATTLES)
        .arg(field_name)
        .query::<u32>(connection)
    {
        return result;
    }
    return 0;
}

// 将某一首歌不同难度的最高分设置进Redis
fn set_music_max_score_to_redis(
    connection: &mut Connection,
    player_id: &str,
    music_id: u32,
    diff: u32,
    max_score: u32,
) {
    let collection_name = format!("zombiedash_{}_{}", music_id, diff);
    let field_name = format!("{}", player_id);

    let mut max_score = max_score;
    if let Ok(result) = redis::cmd("ZSCORE")
        .arg(&collection_name)
        .arg(field_name.clone())
        .query::<u32>(connection)
    {
        if result > max_score {
            max_score = result;
        }
    }

    if let Ok(_) = redis::cmd("ZADD")
        .arg(&collection_name)
        .arg(max_score)
        .arg(field_name)
        .query::<u32>(connection)
    {
        println!("数据插入Redis成功");
    } else {
        println!("ERR 数据插入Redis失败");
    }
}

// 将用户整个游戏的所有最高分设置进Redis
fn set_music_total_score_to_redis(connection: &mut Connection, player_id: &str, total_score: u32) {
    let collection_name = "zombiedash_total_avg_score";
    let field_name = format!("{}", player_id);

    let mut total_score = total_score;
    if let Ok(result) = redis::cmd("ZSCORE")
        .arg(&collection_name)
        .arg(field_name.clone())
        .query::<u32>(connection)
    {
        if result > total_score {
            total_score = result;
        }
    }

    if let Ok(_) = redis::cmd("ZADD")
        .arg(&collection_name)
        .arg(total_score)
        .arg(field_name)
        .query::<u32>(connection)
    {
        println!("总分插入Redis成功");
    } else {
        println!("ERR 总分插入Redis失败");
    }
}

// 打玩一首歌，处理客户端向服务器提交数据
#[post("/musicplayinfo", format = "json", data = "<minfo>")]
pub async fn music_play_rt(
    connector: &State<Connector>,
    minfo: Json<model::ClientableMusicPlayInfo>,
) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        let mut add_total_num_of_battle = false;
        // 简单难度存入排得榜
        if minfo.easy_max_score > 0 {
            set_music_max_score_to_redis(
                &mut redis_connection,
                &minfo.player_id,
                minfo.music_id,
                0,
                minfo.easy_max_score,
            );
            add_number_of_battles(&mut redis_connection, &minfo.player_id, minfo.music_id, 0);
            add_total_num_of_battle = true;
        }
        // 普通难度存入排行榜
        if minfo.normal_max_score > 0 {
            set_music_max_score_to_redis(
                &mut redis_connection,
                &minfo.player_id,
                minfo.music_id,
                1,
                minfo.normal_max_score,
            );

            add_number_of_battles(&mut redis_connection, &minfo.player_id, minfo.music_id, 1);
            add_total_num_of_battle = true;
        }

        // 困难难度存入排行榜
        if minfo.hard_max_score > 0 {
            set_music_max_score_to_redis(
                &mut redis_connection,
                &minfo.player_id,
                minfo.music_id,
                2,
                minfo.hard_max_score,
            );
            add_number_of_battles(&mut redis_connection, &minfo.player_id, minfo.music_id, 2);
            add_total_num_of_battle = true;
        }

        // 所有不为0的歌曲分数平均
        if minfo.total_avg_score > 0 {
            set_music_total_score_to_redis(
                &mut redis_connection,
                &minfo.player_id,
                minfo.total_avg_score,
            );
        }

        if add_total_num_of_battle {
            // 增加总战斗次数
            add_number_of_battles(&mut redis_connection, &minfo.player_id, 0, 0);
        }

        return ApiResponse::empty_ok();
    }

    return ApiResponse::internal_err();
}

// 获取排行榜
#[post("/ranklist", format = "json", data = "<rankinfo>")]
pub async fn rank_list_rt(
    connector: &State<Connector>,
    rankinfo: Json<model::ClientableRankInfo>,
) -> ApiResponse {
    if let Ok(mut redis_connection) = connector.redis.get_connection() {
        let collection_name = if rankinfo.music_id == 0 {
            "zombiedash_total_avg_score".to_string()
        } else {
            // 获取某一首歌某个难度的排行榜
            format!("zombiedash_{}_{}", rankinfo.music_id, rankinfo.diff)
        };

        // 获取排行榜数据
        if let Ok(result) = redis::cmd("ZREVRANGE")
            .arg(collection_name.clone())
            .arg(0)
            .arg(99)
            .arg("WITHSCORES")
            .query::<Vec<(String, String)>>(&mut redis_connection)
        {
            println!("{:?}", result);
            let mut rank_list: Vec<model::RankPlayer> = Vec::new();
            for item in result.iter() {
                let player_id = item.0.clone();
                let score = if let Ok(x) = item.1.parse::<u32>() {
                    x
                } else {
                    0
                };

                // 取出玩家昵称
                let player_name = if let Some(name) =
                    super::get_player_name_from_redis(&mut redis_connection, &player_id)
                {
                    name
                } else {
                    "Unknow".to_string()
                };

                // 取出战斗次数
                let num_of_battles = get_num_of_battles(
                    &mut redis_connection,
                    &player_id,
                    rankinfo.music_id,
                    rankinfo.diff,
                );

                // 取出被膜拜的次数
                let num_of_worship = super::get_worship(&mut redis_connection, &player_id);

                let rank_player = model::RankPlayer {
                    player_id,
                    name: player_name,
                    score,
                    num_of_battles,
                    num_of_worship,
                };
                rank_list.push(rank_player);
            }

            // 取出我的排名
            let my_rank = if let Ok(index) = redis::cmd("ZREVRANK")
                .arg(collection_name)
                .arg(rankinfo.player_id.clone())
                .query::<u32>(&mut redis_connection)
            {
                index + 1
            } else {
                0
            };

            // 取出我的战斗次数
            let my_num_of_battles = get_num_of_battles(
                &mut redis_connection,
                &rankinfo.player_id,
                rankinfo.music_id,
                rankinfo.diff,
            );

            // 取出我被膜拜的次数
            let my_num_of_worship = super::get_worship(&mut redis_connection, &rankinfo.player_id);

            let rrl = model::ResponseRankList {
                rank_list,
                music_id: rankinfo.music_id,
                diff: rankinfo.diff,
                my_rank,
                my_num_of_battles,
                my_num_of_worship,
            };

            return ApiResponse::ok(json!(rrl));
        }
    }

    return ApiResponse::internal_err();
}
