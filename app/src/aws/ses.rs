use crate::AppResult;
use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;

pub async fn send(to: String, subject: String, message: String) -> AppResult<()> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let dest = Destination::builder().to_addresses(to).build();
    let subject_content = Content::builder().data(subject).charset("UTF-8").build();
    let body_content = Content::builder().data(message).charset("UTF-8").build();
    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    client
        .send_email()
        .from_email_address("aki030402@gmail.com")
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}
