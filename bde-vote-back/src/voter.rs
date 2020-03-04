use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum List {
    Blanchon,
    Idk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VotePayload {
    who: List,
    login: String,
}

pub fn vote(payload: VotePayload) {}
