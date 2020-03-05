#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;

use std::path::PathBuf;
use std::sync::Mutex;

use actix_files::NamedFile;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::database::{DbExecutor, NewVote};
use crate::voter::{prepare_vote, VoteOption};
use std::error::Error;
use crate::student_checker::check_login;

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

#[get("/{filename:.*}")]
pub async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/test_db")]
pub async fn test_db_vote(data: web::Data<State>) -> HttpResponse {
    let mut db = data.db.lock().unwrap();

    let res = db.vote(NewVote {
        vote: VoteOption::FourthCompilation,
        login: String::from("malandrl"),
    });

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
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
        return HttpResponse::Unauthorized().json(GenericResponse {
            code: 401,
            error: true,
            message: Some(String::from("Your username is not registered"))
        })
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
        Ok(_) => HttpResponse::Ok().json(GenericResponse {
            code: 200,
            error: false,
            message: Some(String::from("TODO: Show real page")),
        }),
        Err(e) => HttpResponse::Unauthorized().json(GenericResponse {
            code: 401,
            error: true,
            message: Some(e.description().to_owned()),
        }),
    }
}
