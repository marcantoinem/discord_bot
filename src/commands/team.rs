use serde::{Deserialize, Serialize};
use serenity::model::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team(Vec<User>);

impl Team {
    fn new(team: Vec<User>) -> Team {
        Team(team)
    }
}

impl Default for Team {
    fn default() -> Self {
        Team(vec![])
    }
}
