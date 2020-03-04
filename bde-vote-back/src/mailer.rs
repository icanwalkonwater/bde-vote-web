use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{ClientSecurity, SmtpClient, Transport};
use lettre_email::Email;
use std::error::Error;

pub fn send_confirmation_mail(to: String, confirmation_link: String) -> Result<(), impl Error> {
    let email = Email::builder()
        .from(dotenv!("MAIL_FROM"))
        .to(format!("{}@iut2.univ-grenoble-alpes.fr", to))
        .subject("Election BDE - Confirmation du vote")
        .html(format!(
            "
            Vous avez voter pour <b>{}</b><br>
            Veuillez cliquer sur ce lien pour confirmer votre vote: {}<br>
            Ce lien expirera dans 1h.<br>
            <br>
            Si vous n'avez pas voter, ignorez ce mail.",
            to, confirmation_link
        ))
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new((dotenv!("MAIL_SENDGRID_SMTP"), 25), ClientSecurity::None)
        .unwrap()
        .credentials(Credentials::new(
            String::from(dotenv!("MAIL_SENDGRID_USER")),
            String::from(dotenv!("MAIL_SENDGRID_API_KEY")),
        ))
        .authentication_mechanism(Mechanism::Login)
        .smtp_utf8(true)
        .transport();

    let res = mailer.send(email.into())?;

    if res.is_positive() {
        Ok(())
    } else {
        Err(lettre::smtp::error::Error::Client(Box::leak(
            res.code.to_string().into_boxed_str(),
        )))
    }
}
