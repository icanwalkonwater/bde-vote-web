use mysql::prelude::Queryable;
use mysql::{Conn, OptsBuilder};

use crate::voter::VoteOption;

pub struct DbExecutor {
    pub conn: Conn,
}

pub struct NewVote {
    pub login: String,
    pub vote: VoteOption,
}

impl DbExecutor {
    pub fn new_connection() -> simple_error::SimpleResult<Self> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(dotenv!("MYSQL_HOST")))
            .tcp_port(dotenv!("MYSQL_PORT").parse::<u16>().unwrap())
            .db_name(Some(dotenv!("MYSQL_DATABASE")))
            .user(Some(dotenv!("MYSQL_USER")))
            .pass(Some(dotenv!("MYSQL_PASSWORD")));

        Ok(Self {
            conn: try_with!(Conn::new(opts), "Failed to open DB connection"),
        })
    }

    pub fn vote(&mut self, vote: NewVote) -> simple_error::SimpleResult<()> {
        let statement = try_with!(
            self.conn
                .prep("INSERT INTO votes (login, vote) VALUES (:login, :vote);"),
            "Failed to prepare statement"
        );

        Ok(try_with!(
            self.conn.exec_drop(
                statement,
                params! {
                        "login" => vote.login,
                        "vote" => format!("{:?}", vote.vote)
                },
            ),
            "Failed to insert vote into DB"
        ))
    }
}
