use dotenv::dotenv;
use entity::app_user;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_string = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");
    let db = Database::connect(db_string).await.unwrap();

    let user_to_insert = app_user::ActiveModel {
        username: Set("John Doe".to_owned()),
        email: Set("johnDoe@gmail.com".to_owned()),
        ..Default::default()
    };

    user_to_insert.insert(&db).await.unwrap();
    Ok(())
}
