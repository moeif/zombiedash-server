#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use std::sync::{Arc, Mutex};
pub mod data;
mod routes;

pub struct Users {
    pub db: Arc<Mutex<Vec<data::db::User>>>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            db: Arc::new(Mutex::new(vec![])),
        }
    }
}

pub async fn rocket_builder() -> Option<Rocket<Build>> {
    if let Some(connector) = data::connector::init_conn().await {
        Some(
            rocket::build()
                .mount("/", routes![routes::ping::ping_fn])
                .mount(
                    "/api",
                    routes![
                        routes::player::sum_players_rt,
                        routes::player::new_player_rt,
                        routes::player::info_player_rt,
                        routes::musicplay::music_play_rt,
                        routes::musicplay::rank_list_rt,
                        routes::worship::get_worship_rt,
                        routes::worship::post_worship_rt,
                        routes::worship::get_worship_names_rt,
                    ],
                )
                .manage(connector),
        )
    } else {
        None
    }
}

// pub fn rocket_builder() -> rocket::Rocket {
//     rocket::ignite()
//         .attach(SpaceHelmet::default())
//         .mount("/", routes![routes::ping::ping_fn])
//         .mount(
//             "/api",
//             routes![
//                 routes::user::user_list_rt,
//                 routes::user::new_user_rt,
//                 routes::user::info_user_rt,
//                 routes::user::update_user_rt,
//                 routes::user::delete_user_rt,
//                 routes::user::patch_user_rt,
//                 routes::user::id_user_rt
//             ],
//         )
//         .mount("/files", StaticFiles::from("static/"))
//         .manage(Users::new())
// }
