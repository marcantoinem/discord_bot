use std::collections::HashMap;

use crate::events::event::participant::Participant;
use serde::{Deserialize, Serialize};
use serenity::all::UserId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    name: String,
    team: Vec<Participant>,
}

impl Team {
    pub fn new<Text: Into<String>>(name: Text, team: Vec<Participant>) -> Team {
        Team {
            name: name.into(),
            team,
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team::new("Équipe par défault", vec![])
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TeamId(u64);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Teams {
    teams: HashMap<TeamId, Team>,
    participants: HashMap<UserId, TeamId>,
}

impl Default for Teams {
    fn default() -> Self {
        let default_team = Team::new("Équipe #1", vec![]);
        let mut teams = HashMap::new();
        teams.insert(TeamId(0), default_team);
        let participants = HashMap::new();
        Teams {
            teams,
            participants,
        }
    }
}
