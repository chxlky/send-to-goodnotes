use dotenvy_macro::dotenv;
use std::path::PathBuf;
use thiserror::Error;

use lettre::Transport;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Message, MultiPart, SinglePart};
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("too many attachments: {0}, max is 5")]
    TooManyAttachments(usize),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Build(#[from] lettre::error::Error),
    #[error(transparent)]
    Smtp(#[from] lettre::transport::smtp::Error),
    #[error("invalid address: {0}")]
    Addr(String),
}

pub fn send_pdfs(files: Vec<(PathBuf, String)>) -> Result<usize, EmailError> {
    if files.len() > 5 {
        return Err(EmailError::TooManyAttachments(files.len()));
    }

    let smtp_host: &str = dotenv!("SMTP_HOST");
    let smtp_port: &str = dotenv!("SMTP_PORT");
    let from_addr: &str = dotenv!("FROM_EMAIL");
    let to_addr: &str = dotenv!("TO_EMAIL");
    // Remove spaces Google inserts in app passwords for readability
    let app_password: String = dotenv!("APP_PASSWORD").replace(' ', "");

    let creds = Credentials::new(from_addr.to_string(), app_password);

    let port_num: u16 = smtp_port.parse::<u16>().unwrap_or(587);

    let mailer = if port_num == 465 {
        SmtpTransport::relay(smtp_host)?
            .port(465)
            .credentials(creds.clone())
            .build()
    } else {
        SmtpTransport::starttls_relay(smtp_host)?
            .port(port_num)
            .credentials(creds.clone())
            .build()
    };

    let mut parts: Vec<SinglePart> = Vec::new();
    if files.is_empty() {
        parts.push(
            SinglePart::builder()
                .header(ContentType::parse("text/plain; charset=utf-8").unwrap())
                .body(String::from("No attachments provided.")),
        );
    } else {
        for (path, display_name) in &files {
            let data = std::fs::read(path).map_err(EmailError::from)?;
            // Ensure a .pdf extension if user removed it
            let final_name = if !display_name.to_lowercase().ends_with(".pdf") {
                format!("{display_name}.pdf")
            } else {
                display_name.clone()
            };
            parts.push(
                Attachment::new(final_name)
                    .body(data, ContentType::parse("application/pdf").unwrap()),
            );
        }
    }

    let mut iter = parts.into_iter();
    let first = iter.next().expect("at least one part");
    let mut mixed = MultiPart::mixed().singlepart(first);
    for p in iter {
        mixed = mixed.singlepart(p);
    }

    let email = Message::builder()
        .from(
            from_addr
                .parse::<lettre::message::Mailbox>()
                .map_err(|e| EmailError::Addr(e.to_string()))?,
        )
        .to(to_addr
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| EmailError::Addr(e.to_string()))?)
        .subject("PDF files")
        .multipart(mixed)
        .map_err(EmailError::from)?;

    match mailer.send(&email) {
        Ok(resp) => {
            println!("Email sent: code={:?}", resp.code());
            Ok(files.len())
        }
        Err(e) => {
            eprintln!("SMTP send error: {e}");
            Err(EmailError::Smtp(e))
        }
    }
}
