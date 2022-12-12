use crate::events::event::participant::Participant;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team(Vec<Participant>);

impl Team {}

impl Default for Team {
    fn default() -> Self {
        Team(vec![])
    }
}
