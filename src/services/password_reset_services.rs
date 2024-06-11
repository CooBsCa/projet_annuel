use crate::services::users_services;
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
            let password = users_services::reset_password(db, user?)
                .await
                .map_err(|_| DbErr::Custom(("User password has not been reset").to_string()))?;
            // Define the email
            let email = Message::builder()
                .from(format!("ResApp <{}>", host_user).parse().unwrap())
                .to(format!("<{}>", mail_target).parse().unwrap())
                .subject("Reinitialisation de votre mot de passe")
                .body(String::from(format!(
                    "Votre mot de passe à été réinitialisé, voici votre mot de passe temporaire {}",
                    password,
                )))
                .unwrap();

            let creds = Credentials::new(host_user, host_password);

            let mailer = SmtpTransport::relay(&host)
                .unwrap()
                .credentials(creds)
                .build();
            print!("Sending email...");
            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => eprintln!("Could not send email: {:?}", e),
            }
            Ok(())
        }
    }
}
