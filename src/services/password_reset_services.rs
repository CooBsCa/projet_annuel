use crate::api::{reset_password, users};
use crate::services::users_services;
use entity::app_user::{self};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use sea_orm::DbConn;
use sea_orm::DbErr;

pub async fn send_email(db: &DbConn, mail_target: String) -> Result<(), DbErr> {
    let host = std::env::var("EMAIL_HOST").expect("set EMAIL_HOST env variable");
    let host_user = std::env::var("EMAIL_HOST_USER").expect("set EMAIL_HOST_USER env variable");
    let host_password =
        std::env::var("EMAIL_HOST_PASSWORD").expect("set EMAIL_HOST_PASSWORD env variable");

    let user = users_services::user_exists_by_email(db, &mail_target).await;

    match user {
        Err(e) => return Err(e),
        _ => {
            let _ = users_services::reset_password(db, user?).await;
            // Define the email
            let email = Message::builder()
                .from(format!("ResApp <{}>", host_user).parse().unwrap())
                .to(format!("<{}>", mail_target).parse().unwrap())
                .subject("Reinitialisation de votre mot de passe")
                .body(String::from("Hello World, this is a test email from Rust!"))
                .unwrap();

            let creds = Credentials::new(host_user, host_password);

            let mailer = SmtpTransport::relay(&host)
                .unwrap()
                .credentials(creds)
                .build();

            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => eprintln!("Could not send email: {:?}", e),
            }
            Ok(())
        }
    }
}
