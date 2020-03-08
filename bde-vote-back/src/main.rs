#[macro_use]
extern crate dotenv_codegen;

use actix_files;
use actix_web::middleware::Logger;
use env_logger::Env;

use actix_web::web;
use bde_vote_back::database::DbExecutor;
use bde_vote_back::{confirm_vote, index, test_db_vote, vote_for, State, WORKER_AMOUNT};
use std::sync::Mutex;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    dotenv::dotenv().unwrap();
    #[cfg(debug_assertions)]
    env_logger::from_env(Env::default().default_filter_or("trace")).init();
    #[cfg(not(debug_assertions))]
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let state = web::Data::new(State {
        db: Mutex::new(DbExecutor::new_connection().unwrap()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(test_db_vote)
            .service(vote_for)
            .service(confirm_vote)
            .service(index)
            .service(actix_files::Files::new("/js", "./js").show_files_listing())
            .service(actix_files::Files::new("/css", "./css").show_files_listing())
    })
    .workers(WORKER_AMOUNT)
    .bind(format!("{}:{}", dotenv!("HOST"), dotenv!("PORT")))?
    .run()
    .await?;

    println!("Server started on {}:{}", dotenv!("HOST"), dotenv!("PORT"));
    Ok(())
}
