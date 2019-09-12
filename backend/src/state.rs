use crate::db;

use crate::player;
use mongodb::db::Database;
use std::sync::Arc;
use std::sync::Mutex;

pub struct InnerState {
    pub initialized: bool,
    pub player_data: Vec<player::PlayerData>,
}

pub type State = Arc<Mutex<InnerState>>;

impl Default for InnerState {
    fn default() -> Self {
        InnerState {
            initialized: false,
            player_data: vec![],
        }
    }
}

pub fn initialize_state(db: &Database, s: State) {
    let state_ref = s.clone();
    let mut inner_state = state_ref.lock().unwrap();
    if !inner_state.initialized {
        inner_state.player_data = db::initialize_player_data(&db);
        inner_state.initialized = true;
    }
}
