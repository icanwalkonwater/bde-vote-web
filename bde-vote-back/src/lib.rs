#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate mysql;

use std::error::Error;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, post, Result, web};
use serde::{Deserialize, Serialize};

use crate::mailer::send_confirmation_mail;
use crate::voter::VotePayload;

pub mod database;
pub mod mailer;
pub mod voter;

#[cfg(debug_assertions)]
pub const WORKER_AMOUNT: usize = 1;
#[cfg(not(debug_assertions))]
pub const WORKER_AMOUNT: usize = 4;

#[get("/{filename:.*}")]
pub async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/test_mail")]
pub async fn send_email() -> HttpResponse {
    if let Err(e) =
    send_confirmation_mail(String::from("malandrl"), String::from("https://google.fr"))
    {
        HttpResponse::InternalServerError()
            .body(format!("Failed to send email ! {}", e.description()))
    } else {
        HttpResponse::Ok().body("Email sent successfully !")
    }
}

#[post("/vote")]
pub async fn vote_for(payload: web::Json<VotePayload>) -> HttpResponse {
    let payload = payload.into_inner();
    println!("{:?}", payload);

    HttpResponse::Ok().json(payload)
}
