use std::{
    collections::{hash_map::Iter, HashMap},
    fmt,
};

use super::{events::Events, participant::Participant};
use serde::{Deserialize, Serialize};
use serenity::{
    all::{ScheduledEventId, UserId},
    builder::*,
    prelude::Context,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub name: String,
    description: String,
    team: Vec<Participant>,
}

impl Team {
    pub fn new<Text: Into<String>>(name: Text, description: Text, team: Vec<Participant>) -> Team {
        Team {
            name: name.into(),
            description: description.into(),
            team,
        }
    }
}

impl Default for Team {
    fn default() -> Self {
        Team::new("Par défault", "test", vec![])
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "**{}**: {}", self.name, self.description)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TeamId(pub u64);

impl fmt::Display for TeamId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Teams {
    teams: HashMap<TeamId, Team>,
    participants: HashMap<UserId, TeamId>,
    max_participants: Option<u32>,
}

impl Teams {
    pub fn add_team<Text: Into<String>>(&mut self, name: Text, description: Text) {
        let team = Team::new(name, description, vec![]);
        let team_id = TeamId(self.teams.len() as u64);
        self.teams.insert(team_id, team);
    }
    pub fn iter(&self) -> Iter<'_, TeamId, Team> {
        self.teams.iter()
    }
    pub fn get_team(&self, id: &TeamId) -> Option<Team> {
        self.teams.get(id).cloned()
    }
    pub fn get_participants_team(&self, id: &UserId) -> Option<Team> {
        let team_id = self.participants.get(id).cloned()?;
        self.get_team(&team_id)
    }
    pub async fn menu(ctx: &Context, event_id: ScheduledEventId) -> CreateSelectMenu {
        let events_lock = Events::get_lock(ctx).await;
        let events = events_lock.read().await;
        let event = events.get(&event_id).unwrap();
        let options = event
            .teams
            .iter()
            .map(|(id, team)| CreateSelectMenuOption::new(team.name.clone(), id.to_string()))
            .collect();
        let select_menu = CreateSelectMenuKind::String { options };
        CreateSelectMenu::new("team", select_menu)
    }
}

impl Default for Teams {
    fn default() -> Self {
        let default_team = Team::default();
        let mut teams = HashMap::new();
        teams.insert(TeamId(0), default_team);
        let participants = HashMap::new();
        Teams {
            teams,
            participants,
            max_participants: None,
        }
    }
}

impl fmt::Display for Teams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.teams.iter().fold(Ok(()), |result, (_, team)| {
            result.and_then(|_| writeln!(f, "{}", team))
        })
    }
}
