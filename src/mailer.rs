use dotenv::dotenv;
use lettre::{smtp::authentication::Credentials, SmtpClient, SmtpTransport, Transport};
use lettre_email::EmailBuilder;

use crate::error::{ErrorKind, Errors};

#[allow(dead_code)]
pub struct Mailer<'a> {
    email: String,
    password: String,
    to_email: &'a str,
    mailer: SmtpTransport,
}

impl<'a> Mailer<'a> {
    pub fn new(to_email: &'a str) -> Result<Mailer<'a>, Errors<'a>> {
        dotenv().ok();
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();
        let creds = Credentials::new(email.clone(), password.clone());
        let mailer = match SmtpClient::new_simple("smtp.gmail.com") {
            Ok(val) => val.credentials(creds).transport(),
            Err(_) => {
                return Err(Errors::new(
                    ErrorKind::MailError,
                    "Cannot Connect to SMTP Server",
                ))
            }
        };

        Ok(Mailer {
            email,
            password,
            to_email,
            mailer,
        })
    }

    pub fn send(&mut self, link: &'a str) -> Result<(), Errors<'a>> {
        let mail = EmailBuilder::new();
        let body = format!(
            r#"<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta http-equiv='X-UA-Compatible' content='IE=edge'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Document</title>
        </head>
        <body>
            Uptime Bot The keyword is down vistit link <a href='{}'>click here</a>
        </body>
        </html>"#,
            link
        );
        let mail = match mail
            .from(self.email.as_str())
            .to(self.to_email)
            .subject("ALERT")
            .html(body)
            // .body(body)
            .build()
        {
            Ok(val) => val,
            Err(_) => {
                return Err(Errors::new(
                    ErrorKind::MailError,
                    "Cannot Build Email Content",
                ))
            }
        };

        match self.mailer.send(mail.into()) {
            Ok(_) => {
                println!("Email Send Successfully, Alerted: {:^50}", self.to_email);
                Ok(())
            }
            Err(e) => {
                println!("{}", e.to_string());
                Err(Errors::new(ErrorKind::MailError, "Cannot Send e-mail"))
            }
        }
    }
}
