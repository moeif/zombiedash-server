use zombiedash_server::rocket_builder;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if let Some(rocket_builder) = rocket_builder().await {
        rocket_builder.launch().await
    } else {
        Ok(())
    }
}
