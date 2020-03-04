use std::error::Error;

use actix::{Actor, Message, SyncContext};
use actix::prelude::*;
use mysql::{Conn, OptsBuilder};
use mysql::prelude::Queryable;

use crate::voter::List;

pub struct DbExecutor {
    pub conn: Conn,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct PrepareVote {
    vote: List,
    login: String,
    confirmation_token: String,
}

impl Message for PrepareVote {
    type Result = mysql::Result<()>;
}

impl Handler<PrepareVote> for DbExecutor {
    type Result = mysql::Result<()>;

    fn handle(&mut self, msg: PrepareVote, _: &mut Self::Context) -> Self::Result {
        let statment = self.conn.prep("
                INSERT INTO votes (login, vote, confirmation_token)
                VALUES (:login, :vote, :confirmation_token);
        ")?;

        self.conn.exec_drop(statment, params! {
            "login" => msg.login,
            "vote" => format!("{:?}", msg.vote),
            "confirmation_token" => msg.confirmation_token,
        })
    }
}

pub fn create_sql_connection() -> mysql::Result<Conn> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(dotenv!("MYSQL_HOST")))
        .tcp_port(dotenv!("MYSQL_PORT").parse::<u16>().unwrap())
        .db_name(Some(dotenv!("MYSQL_DATABASE")))
        .user(Some(dotenv!("MYSQL_USER")))
        .pass(Some(dotenv!("MYSQL_PASSWORD")));

    Conn::new(opts)
}
