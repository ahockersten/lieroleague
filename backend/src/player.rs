use serde::{Serialize, Deserialize};
use eventsourcing_derive::Event;

#[derive(Debug)]
pub enum PlayerCommand {
  Create()
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("events://github.com/ahockersten/lieroleague/player")]
enum PlayerEvent {
  PlayerCreated,
}

#[derive(Debug)]
struct PlayerData {
  id: u64,
  real_name: Option<String>,
  nick_name: String,
  color: PlayerColor,

}

#[derive(Debug)]
struct PlayerColor {
  r: u8,
  g: u8,
  b: u8
}
