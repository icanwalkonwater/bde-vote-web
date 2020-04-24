#[macro_use]
extern crate dotenv_codegen;

use actix_web::middleware::Logger;
use env_logger::Env;

use actix_web::web;
use bde_vote_back::database::DbExecutor;
use bde_vote_back::{confirm_vote, index, file, vote_for, State, WORKER_AMOUNT};
use std::sync::Mutex;
use actix_cors::Cors;

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
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .finish()
            )
            .service(vote_for)
            .service(confirm_vote)
            .service(index)
            .service(file)
    })
    .workers(WORKER_AMOUNT)
    .bind(format!("{}:{}", dotenv!("HOST"), dotenv!("PORT")))?
    .run()
    .await?;

    println!("Server started on {}:{}", dotenv!("HOST"), dotenv!("PORT"));
    Ok(())
}
