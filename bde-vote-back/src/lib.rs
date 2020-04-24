#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate simple_error;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;

use actix_files::NamedFile;
use actix_web::{get, http, HttpRequest, HttpResponse, post, Result, web};
use serde::{Deserialize, Serialize};

use crate::database::DbExecutor;
use crate::student_checker::check_login;
use crate::voter::{prepare_vote, VoteOption};

pub mod database;
pub mod mailer;
pub mod student_checker;
pub mod voter;

#[cfg(debug_assertions)]
pub const WORKER_AMOUNT: usize = 1;
#[cfg(not(debug_assertions))]
pub const WORKER_AMOUNT: usize = 4;

pub struct State {
    pub db: Mutex<DbExecutor>,
}

#[derive(Serialize)]
pub struct GenericResponse {
    code: usize,
    error: bool,
    message: Option<String>,
}

#[get("/")]
pub async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("index.html".parse::<PathBuf>().unwrap())?)
}

#[get("/{filename:.*}")]
pub async fn file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VotePayload {
    who: VoteOption,
    login: String,
}

#[post("/vote")]
pub async fn vote_for(payload: web::Json<VotePayload>) -> HttpResponse {
    let payload = payload.into_inner();

    if !check_login(&payload.login) {
        info!(
            "Vote: {} is not on the username whitelist !",
            &payload.login
        );
        return HttpResponse::Unauthorized().json(GenericResponse {
            code: 401,
            error: true,
            message: Some(String::from("Your username is not registered")),
        });
    }

    match prepare_vote(payload) {
        Ok(_) => HttpResponse::Ok().json(GenericResponse {
            code: 200,
            error: false,
            message: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(GenericResponse {
            code: 500,
            error: true,
            message: Some(e.description().to_owned()),
        }),
    }
}

#[derive(Deserialize)]
pub struct ConfirmVoteInfo {
    token: String,
}

#[get("/confirm_vote")]
pub async fn confirm_vote(
    info: web::Query<ConfirmVoteInfo>,
    data: web::Data<State>,
) -> HttpResponse {
    let token = info.token.clone();
    let mut db = data.db.lock().unwrap();

    match voter::confirm_vote(&mut db, token) {
        Ok(_) => HttpResponse::Found()
            .header(http::header::LOCATION, "/confirmed.html")
            .finish()
            .into_body(),
        Err(e) => HttpResponse::Unauthorized().json(GenericResponse {
            code: 401,
            error: true,
            message: Some(e.description().to_owned()),
        }),
    }
}
