use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    member: Member,
}
