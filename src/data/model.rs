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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicPlayInfo {
    pub play_id: String,
    pub player_id: String,
    pub music_id: u32,
    pub diff: u32,
    pub max_score: u32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl MusicPlayInfo {
    pub fn new(player_id: String, music_id: u32, diff: u32, max_score: u32) -> Self {
        Self {
            play_id: format!("{}_{}_{}", player_id, music_id, diff),
            player_id,
            music_id,
            diff,
            max_score,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    pub fn from_clientable(cinfo: ClientableMusicPlayInfo) -> Self {
        MusicPlayInfo::new(cinfo.player_id, cinfo.music_id, cinfo.diff, cinfo.max_score)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientableMusicPlayInfo {
    pub player_id: String,
    pub music_id: u32,
    pub diff: u32,
    pub max_score: u32,
}
