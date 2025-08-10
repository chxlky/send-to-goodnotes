use std::path::PathBuf;
use thiserror::Error;

use lettre::Transport;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Message, MultiPart, SinglePart};
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;

use crate::config::EmailSettings;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("too many attachments: {0}, max is 5")]
    TooManyAttachments(usize),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Build(#[from] lettre::error::Error),
    #[error(transparent)]
    Smtp(#[from] lettre::transport::smtp::Error),
    #[error("invalid address: {0}")]
    Addr(String),
    #[error("invalid port number: {0}")]
    InvalidPort(String),
    #[error("settings not configured properly")]
    IncompleteSettings,
}

pub fn send_pdfs(
    files: Vec<(PathBuf, String)>,
    settings: &EmailSettings,
) -> Result<usize, EmailError> {
    if files.len() > 5 {
        return Err(EmailError::TooManyAttachments(files.len()));
    }

    // Validate settings
    if settings.smtp_host.is_empty()
        || settings.from_email.is_empty()
        || settings.to_email.is_empty()
        || settings.app_password.is_empty()
    {
        return Err(EmailError::IncompleteSettings);
    }

    let app_password = settings.app_password.replace(' ', "");
    let creds = Credentials::new(settings.from_email.clone(), app_password);

    let port_num: u16 = settings
        .smtp_port
        .parse()
        .map_err(|_| EmailError::InvalidPort(settings.smtp_port.clone()))?;

    let mailer = if port_num == 465 {
        SmtpTransport::relay(&settings.smtp_host)?
            .port(465)
            .credentials(creds)
            .build()
    } else {
        SmtpTransport::starttls_relay(&settings.smtp_host)?
            .port(port_num)
            .credentials(creds)
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
            let data = std::fs::read(path)?;

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
            settings
                .from_email
                .parse::<lettre::message::Mailbox>()
                .map_err(|e| EmailError::Addr(e.to_string()))?,
        )
        .to(settings
            .to_email
            .parse::<lettre::message::Mailbox>()
            .map_err(|e| EmailError::Addr(e.to_string()))?)
        .subject("PDF files")
        .multipart(mixed)?;

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
