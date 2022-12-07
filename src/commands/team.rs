use serenity::model::user::User;

#[derive(Clone, Debug)]
pub struct Team(Option<Vec<User>>);

impl Team {
    fn new(team: Vec<User>) -> Team {
        Team(Some(team))
    }
}

impl Default for Team {
    fn default() -> Self {
        Team(None)
    }
}
