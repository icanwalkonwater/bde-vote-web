use std::fmt;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::database::{DbExecutor, NewVote};
use crate::mailer::send_confirmation_mail;
use crate::VotePayload;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum VoteOption {
    FourthCompilation,
    AlphaOs,
}

impl fmt::Display for VoteOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VoteOption::FourthCompilation => f.write_str("4ème Compilation")?,
            VoteOption::AlphaOs => f.write_str("αOS")?,
        };

        Ok(())
    }
}

impl FromStr for VoteOption {
    type Err = simple_error::SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FourthCompilation" => Ok(VoteOption::FourthCompilation),
            "AlphaOs" => Ok(VoteOption::AlphaOs),
            _ => bail!("Unknown vote: {}", s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmationToken {
    iss: String,
    sub: VoteOption,
    exp: u64,
}

pub fn prepare_vote(payload: VotePayload) -> simple_error::SimpleResult<()> {
    let claims = ConfirmationToken {
        iss: payload.login.clone(),
        sub: payload.who,
        exp: (SystemTime::now() + Duration::from_secs(60 * 60))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
    )
    .unwrap();

    Ok(try_with!(
        send_confirmation_mail(
            payload.login,
            payload.who,
            format!("{}?token={}", dotenv!("VOTE_CONFIRM_LINK_PREFIX"), token),
        ),
        "Failed to send mail !"
    ))
}

pub fn confirm_vote(db: &mut DbExecutor, token: String) -> simple_error::SimpleResult<()> {
    let validation = Validation {
        validate_exp: true,
        algorithms: vec![Algorithm::HS512],
        ..Validation::default()
    };

    let res = jsonwebtoken::decode::<ConfirmationToken>(
        &token,
        &DecodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes()),
        &validation,
    );

    if let Err(e) = res {
        bail!("Failed to decode token because of: {:?}", e);
    }

    let TokenData {
        claims:
            ConfirmationToken {
                iss: login,
                sub: vote,
                ..
            },
        ..
    } = res.unwrap(); // Now we can unwrap safely

    let res = db.vote(NewVote { vote, login: login.clone() });
    info!("{} voted for {:?}", login, vote);

    res
}
