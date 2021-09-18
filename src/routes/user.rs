// use crate::data::db::{InsertableUser, ResponseUser, User, UserPassword};
// use mongodb::bson::{self, doc};
// use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
// use mongodb::Database;
// use rocket::http::{ContentType, Status};
// use rocket::request::Request;
// use rocket::response::{self, Responder, Response};
// use rocket::serde::json::serde_json::json;
// use rocket::serde::json::Json;
// use rocket::serde::json::Value as JsonValue;
// use rocket::serde::uuid::Uuid;
// use rocket::State;
// use rocket::*;

// const COLLECTION: &str = "users";

// #[get("/users")]
// pub async fn user_list_rt(database: &State<Database>) -> ApiResponse {
//     let user_coll = database.collection::<User>(COLLECTION);
//     match user_coll.count_documents(None, None).await {
//         Ok(res) => ApiResponse::ok(json!([res])),
//         Err(_) => ApiResponse::internal_err(),
//     }
// }

// #[post("/users", format = "json", data = "<user>")]
// pub async fn new_user_rt(database: &State<Database>, user: Json<InsertableUser>) -> ApiResponse {
//     let user_coll = database.collection(COLLECTION);
//     if let Ok(find_one) = user_coll
//         .find_one(Some(doc! {"email": user.email.clone()}), None)
//         .await
//     {
//         if let Some(_) = find_one {
//             return ApiResponse::ok(json! (
//                 {"code": 1, "msg": format!("Email {0} already exists.", user.email)}
//             ));
//         } else {
//             let new_user = User::from_insertable(user.clone());
//             if let Ok(serialized) = bson::to_bson(&new_user) {
//                 if let Some(document) = serialized.as_document() {
//                     if let Ok(_) = user_coll.insert_one(document.to_owned(), None).await {
//                         return ApiResponse::ok(json!(
//                             {"code": 0, "msg": "success", "id": new_user.id}
//                         ));
//                     }
//                 }
//             }
//         }
//     }

//     return ApiResponse::internal_err();
// }

// #[get("/users/<id>")]
// pub async fn info_user_rt(database: &State<Database>, id: Uuid) -> ApiResponse {
//     let user_coll = database.collection::<User>(COLLECTION);
//     if let Ok(find_one) = user_coll
//         .find_one(Some(doc! {"_id": id.to_string() }), None)
//         .await
//     {
//         if let Some(found_user) = find_one {
//             return ApiResponse::ok(json!(ResponseUser::from_user(&found_user)));
//         } else {
//             println!("find_one: {:?}", find_one);
//         }
//     } else {
//         println!("Not found user with id: {}", id);
//     }

//     return ApiResponse::internal_err();
// }

// #[put("/users/<id>", format = "json", data = "<user>")]
// pub async fn update_user_rt(
//     database: &State<Database>,
//     user: Json<InsertableUser>,
//     id: Uuid,
// ) -> ApiResponse {
//     let user_coll = database.collection::<User>(COLLECTION);
//     let id = id.to_string();
//     match user_coll
//         .find_one(Some(doc! {"_id": id.clone() }), None)
//         .await
//     {
//         Ok(find_one) => {
//             if let Some(mut found_user) = find_one {
//                 if found_user.match_password(&user.password) {
//                     let insertable = found_user.update_user(&user.name, &user.email);
//                     if let Ok(serialized) = bson::to_bson(&insertable) {
//                         if let Some(document) = serialized.as_document() {
//                             let mut opt = FindOneAndUpdateOptions::default();
//                             opt.return_document = Some(ReturnDocument::After);
//                             if let Ok(updated_one) = user_coll
//                                 .find_one_and_update(
//                                     doc! {"_id": id.clone()},
//                                     document.to_owned(),
//                                     Some(opt),
//                                 )
//                                 .await
//                             {
//                                 if let Some(updated_user) = updated_one {
//                                     return ApiResponse::ok(json!(ResponseUser::from_user(
//                                         &updated_user
//                                     )));
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             return ApiResponse::internal_err();
//         }
//         Err(_) => return ApiResponse::internal_err(),
//     }
// }

// #[delete("/users/<id>", format = "json", data = "<user>")]
// pub async fn delete_user_rt(
//     database: &State<Database>,
//     user: Json<UserPassword>,
//     id: Uuid,
// ) -> ApiResponse {
//     // let mut v = userdb.db.lock().unwrap();
//     // let users = &mut *v;
//     // let pos = users
//     //     .iter()
//     //     .position(|x| x.id.to_string() == id.to_string());
//     // match pos {
//     //     Some(p) => {
//     //         if v[p].match_password(&user.password) {
//     //             let u = v[p].clone();
//     //             v.remove(p);
//     //             ApiResponse::ok(json!(ResponseUser::from_user(&u)))
//     //         } else {
//     //             ApiResponse::err(json!("user not authenticated!"))
//     //         }
//     //     }
//     //     None => ApiResponse::err(json!(format!("id {} not found", id))),
//     // }
//     return ApiResponse::internal_err();
// }

// #[patch("/users/<id>", format = "json", data = "<user>")]
// pub async fn patch_user_rt(
//     database: &State<Database>,
//     user: Json<UserPassword>,
//     id: Uuid,
// ) -> ApiResponse {
//     // let mut v = userdb.db.lock().unwrap();
//     // let users = &mut *v;
//     // let pos = users
//     //     .iter()
//     //     .position(|x| x.id.to_string() == id.to_string());
//     // match pos {
//     //     Some(p) => {
//     //         if v[p].match_password(&user.password) {
//     //             match &user.new_password {
//     //                 Some(passw) => {
//     //                     v[p].update_password(&passw);
//     //                     ApiResponse::ok(json!("Password updated"))
//     //                 }
//     //                 None => ApiResponse::err(json!("Password not provided")),
//     //             }
//     //         } else {
//     //             ApiResponse::err(json!("user not authenticated"))
//     //         }
//     //     }
//     //     None => ApiResponse::err(json!(format!("id {} not found", id))),
//     // }

//     return ApiResponse::internal_err();
// }

// #[get("/users/<email>", rank = 2)]
// pub async fn id_user_rt(database: &State<Database>, email: String) -> ApiResponse {
//     // let mut v = userdb.db.lock().unwrap();
//     // let users = &mut *v;
//     // let pos = users.iter().position(|x| x.email == email);
//     // match pos {
//     //     Some(p) => ApiResponse::ok(json!(ResponseUser::from_user(&v[p]))),
//     //     None => ApiResponse::err(json!(format!("user {} not found", email))),
//     // }
//     let user_coll = database.collection::<User>(COLLECTION);
//     if let Ok(find_one) = user_coll
//         .find_one(Some(doc! { "email": email }), None)
//         .await
//     {
//         println!("findone: {:?}", find_one);
//         if let Some(found_user) = find_one {
//             return ApiResponse::ok(json!(ResponseUser::from_user(&found_user)));
//         }
//     }

//     return ApiResponse::internal_err();
// }
