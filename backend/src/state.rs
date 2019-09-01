use crate::db;
use crate::db::LieroLeagueDb;
use crate::player;

pub struct State {
    pub initialized: bool,
    pub player_data: Vec<player::PlayerData>,
}

impl Default for State {
    fn default() -> Self {
        State {
            initialized: false,
            player_data: vec![],
        }
    }
}

pub fn initialize_state(mut s: State) {
    if !s.initialized {
        s.initialized = true;
    }
}
