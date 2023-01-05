use std::fmt;

use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participant {
    pub id: UserId,
    pub name: String,
}

impl fmt::Display for Participant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.mention())
    }
}

impl From<User> for Participant {
    fn from(user: User) -> Participant {
        Participant {
            id: user.id,
            name: user.name,
        }
    }
}
