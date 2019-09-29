use crate::db;
use crate::player;

use mongodb::db::Database;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

pub struct InnerState {
    pub initialized: bool,
    pub player_data: HashMap<Uuid, player::PlayerData>,
}

pub type State = Arc<Mutex<InnerState>>;

impl Default for InnerState {
    fn default() -> Self {
        InnerState {
            initialized: false,
            player_data: HashMap::new(),
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
