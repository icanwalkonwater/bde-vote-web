use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{ClientSecurity, SmtpClient, Transport};
use lettre_email::Email;

use crate::voter::VoteOption;

pub fn send_confirmation_mail(
    to: String,
    list: VoteOption,
    confirmation_link: String,
) -> simple_error::SimpleResult<()> {
    let email = Email::builder()
        .from(dotenv!("MAIL_FROM"))
        .to(format!("{}@iut2.univ-grenoble-alpes.fr", to))
        .subject("Election BDE - Confirmation du vote")
        .html(format!(
            "
            Vous avez voter pour <b>{}</b>.<br>
            <br>
            Veuillez cliquer sur ce lien pour confirmer votre vote:<br>
            {}<br>
            Ce lien expirera dans 1h.<br>
            <br>
            Si vous n'avez pas voter, ignorez ce mail.",
            list, confirmation_link
        ))
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new(
        (
            dotenv!("MAIL_SENDGRID_SMTP"),
            dotenv!("MAIL_SENDGRID_SMTP_PORT").parse::<u16>().unwrap(),
        ),
        ClientSecurity::None,
    )
    .unwrap()
    .credentials(Credentials::new(
        String::from(dotenv!("MAIL_SENDGRID_USER")),
        String::from(dotenv!("MAIL_SENDGRID_API_KEY")),
    ))
    .authentication_mechanism(Mechanism::Login)
    .smtp_utf8(true)
    .transport();

    let res = try_with!(mailer.send(email.into()), "Failed to send mail");

    if res.is_positive() {
        Ok(())
    } else {
        bail!(
            "Failed with code {} and message: {}",
            res.code,
            res.message.join(", ")
        )
    }
}
