use api::server::start_server;
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

pub mod api;
pub mod dto;
pub mod middleware;
pub mod services;
pub mod utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_string = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");
    let db = Database::connect(db_string).await.unwrap();

    // Run the migration
    Migrator::up(&db, None).await.unwrap();

    //Start http server
    start_server(db).await;
    // let user_to_insert = app_user::ActiveModel {
    //     username: Set("John Doe".to_owned()),
    //     email: Set("johnDoe@gmail.com".to_owned()),
    //     ..Default::default()
    // };

    // user_to_insert.insert(&db).await.unwrap();
    Ok(())
}
