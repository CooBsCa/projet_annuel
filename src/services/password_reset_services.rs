use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_email(
    mail_target: String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Define the email
    let email = Message::builder()
        .from("ResApp <no_reply@demomailtrap.com>".parse().unwrap())
        .to(format!("<{}>", mail_target).parse().unwrap())
        .subject("Rust Email")
        .body(String::from("Hello World, this is a test email from Rust!"))
        .unwrap();

    let creds = Credentials::new(
        "api".to_string(),
        "6a6707b883c7dad8d5115b1812b19902".to_string(),
    );

    let mailer = SmtpTransport::starttls_relay("live.smtp.mailtrap.io")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
    Ok(())
}
