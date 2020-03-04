#[macro_use]
extern crate dotenv_codegen;

use std::path::PathBuf;

use actix::{Addr, SyncArbiter};
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, HttpRequest, Result};
use actix_web::middleware::Logger;
use env_logger::Env;
use mysql::{Conn, OptsBuilder};

use bde_vote_back::{index, send_email, vote_for, WORKER_AMOUNT};
use bde_vote_back::database::{create_sql_connection, DbExecutor};

struct State {
    db: Addr<DbExecutor>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    dotenv::dotenv().unwrap();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let sys = actix::System::new("bde-vote-backend");

    let addr = SyncArbiter::start(1, || DbExecutor {
        conn: create_sql_connection().expect("Failed to connect to the database"),
    });

    HttpServer::new(move || {
        App::new()
            .data(State { db: addr.clone() })
            .wrap(Logger::default())
            .service(send_email)
            .service(vote_for)
            .service(index)
            .service(fs::Files::new("/js", "./js").show_files_listing())
            .service(fs::Files::new("/css", "./css").show_files_listing())
    })
        .workers(WORKER_AMOUNT)
        .bind(format!("{}:{}", dotenv!("HOST"), dotenv!("PORT")))?
        .run()
        .await?;

    println!("Server started on {}:{}", dotenv!("HOST"), dotenv!("PORT"));
    let _ = sys.run()?;
    Ok(())
}
