use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

fn main() {
    tracing_subscriber::fmt::init();

    // The html we want to send.
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hello from Lettre!</title>
</head>
<body>
    <div style="display: flex; flex-direction: column; align-items: center;">
        <h2 style="font-family: Arial, Helvetica, sans-serif;">Hello from Lettre!</h2>
        <h4 style="font-family: Arial, Helvetica, sans-serif;">A mailer library for Rust</h4>
    </div>
</body>
</html>"#;

    // Build the message.
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Hello from Lettre!")
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
        .expect("failed to build email");

    //let email = Message::builder()
    //    .from("NoBody <nobody@domain.tld>".parse().unwrap())
    //    .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
    //    .to("Hei <hei@domain.tld>".parse().unwrap())
    //    .subject("Happy new year")
    //    .header(ContentType::TEXT_PLAIN)
    //    .body(String::from("Be happy!"))
    //    .unwrap();

    //let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

    // Open a local connection on port 25
    let mailer = SmtpTransport::builder_dangerous("localhost")
        .port(2525)
        //.credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
