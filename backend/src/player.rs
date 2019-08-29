use serde::{Serialize, Deserialize};
use eventsourcing::AggregateState;
use eventsourcing_derive::Event;

#[derive(Debug)]
enum PlayerCommand {
  Create {
    real_name: String,
    email: Email,
    password: String,
    salt: String,
    nick_name: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    time_zone: Option<i8>,
    location: Option<Country>,
    locale: Locale,
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("events://github.com/ahockersten/lieroleague/player")]
enum PlayerEvent {
  PlayerCreated {
    id: u64,
    real_name: String,
    email: Email,
    password: String,
    salt: String,
    nick_name: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    time_zone: Option<i8>,
    location: Option<Country>,
    locale: Locale,
  },
}

type Email = String;
type Nationality = String;
type Country = String;
type Locale = String;

#[derive(Debug)]
struct PlayerData {
  id: u64,
  real_name: String,
  email: Email,
  password: String,
  salt: String,
  nick_name: String,
  color: PlayerColor,
  nationality: Option<Nationality>,
  time_zone: Option<i8>,
  location: Option<Country>,
  locale: Locale,
  generation: u64,
}

impl AggregateState for PlayerData {
  fn generation(&self) -> u64 {
    self.generation
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PlayerColor {
  r: u8,
  g: u8,
  b: u8
}
