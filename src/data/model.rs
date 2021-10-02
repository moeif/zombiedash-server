use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub player_id: String,
    pub name: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Player {
    pub fn new(player_id: String, name: String) -> Self {
        Self {
            player_id,
            name,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn from_clientable(cplayer: ClientablePlayer) -> Self {
        Player::new(cplayer.player_id, cplayer.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientablePlayer {
    pub player_id: String,
    pub name: String,
}

impl ClientablePlayer {
    pub fn new(player_id: String, name: String) -> Self {
        Self { player_id, name }
    }

    pub fn from_player(player: Player) -> Self {
        Self {
            player_id: player.player_id,
            name: player.name,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct MusicPlayInfo {
//     pub play_id: String,
//     pub player_id: String,
//     pub music_id: u32,
//     pub easy_max_score: u32,
//     pub normal_max_score: u32,
//     pub hard_max_score: u32,
//     pub created: DateTime<Utc>,
//     pub updated: DateTime<Utc>,
// }

// impl MusicPlayInfo {
//     pub fn new(
//         player_id: String,
//         music_id: u32,
//         easy_max_score: u32,
//         normal_max_score: u32,
//         hard_max_score: u32,
//     ) -> Self {
//         Self {
//             play_id: format!("{}_{}_{}", player_id, music_id, diff),
//             player_id,
//             music_id,
//             easy_max_score,
//             normal_max_score,
//             hard_max_score,
//             created: Utc::now(),
//             updated: Utc::now(),
//         }
//     }

//     pub fn from_clientable(cinfo: ClientableMusicPlayInfo) -> Self {
//         MusicPlayInfo::new(
//             cinfo.player_id,
//             cinfo.music_id,
//             cinfo.easy_max_score,
//             cinfo.normal_max_score,
//             cinfo.hard_max_score,
//         )
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientableMusicPlayInfo {
    pub player_id: String,
    pub music_id: u32,
    pub easy_max_score: u32,
    pub normal_max_score: u32,
    pub hard_max_score: u32,
    pub total_avg_score: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientableRankInfo {
    pub player_id: String,
    pub music_id: u32, // 如果music_id为0，则返回总榜
    pub diff: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RankPlayer {
    pub player_id: String,
    pub name: String,
    pub score: u32,
    pub num_of_battles: u32, // 战斗次数，总的或者当前歌曲及难度的
    pub num_of_worship: u32, // 被膜拜次数
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseRankList {
    pub rank_list: Vec<RankPlayer>,
    pub music_id: u32,
    pub diff: u32,
    pub my_rank: u32,
    pub my_num_of_battles: u32,
    pub my_num_of_worship: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientableWorship {
    pub src_player_id: String,
    pub dst_player_id: String,
}
