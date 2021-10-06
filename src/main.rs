#[macro_use]
extern crate log4rs;
use zombiedash_server::rocket_builder;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    if let Some(rocket_builder) = rocket_builder().await {
        rocket_builder.launch().await
    } else {
        Ok(())
    }
}
