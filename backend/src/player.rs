use rocket::{Route, post, routes};
use crate::db;
use eventsourcing::Kind::CommandFailure;
use serde::{Serialize, Deserialize};
use eventsourcing::{Aggregate, AggregateState, Error};
use eventsourcing_derive::Event;
use uuid::Uuid;
use passwords::hasher;
use rocket_contrib::json::Json;
use std::mem;

#[derive(Debug)]
pub enum PlayerCommand {
  Create {
    real_name: String,
    email: Email,
    password: String,
    nick_name: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    time_zone: Option<i8>,
    location: Option<Country>,
    locale: Locale,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("events://github.com/ahockersten/lieroleague/player")]
pub enum PlayerEvent {
  Created {
    id: Uuid,
    real_name: String,
    email: Email,
    salted_password: [i8; 24],
    salt: [i8; 16],
    nick_name: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    time_zone: Option<i8>,
    location: Option<Country>,
    locale: Locale,
  },
}


impl From<PlayerCommand> for PlayerEvent {
  fn from(source: PlayerCommand) -> Self {
    match source {
      PlayerCommand::Create {
        real_name,
        email,
        password,
        nick_name,
        color,
        nationality,
        time_zone,
        location,
        locale
      } => {
        let salt = hasher::gen_salt();
        let salted_password = hasher::bcrypt(12, &salt, &password).unwrap();  // TODO: this could fail
        unsafe {
          // No way to store unsigned values in BSON, do a conversion here instead
          let transmuted_salt = mem::transmute::<[u8; 16], [i8; 16]>(salt);
          let transmuted_salted_password = mem::transmute::<[u8; 24], [i8; 24]>(salted_password);

          PlayerEvent::Created {
            id: Uuid::new_v4(),
            real_name,
            email,
            salted_password: transmuted_salted_password,
            salt: transmuted_salt,
            nick_name,
            color,
            nationality,
            time_zone,
            location,
            locale
          }
        }
      }
    }
  }
}

type Email = String;
type Nationality = String;
type Country = String;
type Locale = String;

#[derive(Debug)]
pub struct PlayerData {
  id: Uuid,
  real_name: String,
  email: Email,
  salted_password: [u8; 24],
  salt: [u8; 16],
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
pub struct PlayerColor {
  #[serde(with = "bson::compat::u2f")]
  r: u8,
  #[serde(with = "bson::compat::u2f")]
  g: u8,
  #[serde(with = "bson::compat::u2f")]
  b: u8
}

struct Player;

impl Aggregate for Player {
  type Event = PlayerEvent;
  type Command = PlayerCommand;
  type State = PlayerData;

  fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State, Error> {
    let data = match evt {
      PlayerEvent::Created {
        id,
        real_name,
        email,
        salted_password,
        salt,
        nick_name,
        color,
        nationality,
        time_zone,
        location,
        locale
      } => {
        unsafe {
          // These were stored signed in the database (see above),
          // so transmute them back here
          let transmuted_salt = mem::transmute::<[i8; 16], [u8; 16]>(salt);
          let transmuted_salted_password = mem::transmute::<[i8; 24], [u8; 24]>(salted_password);
          PlayerData {
            id,
            real_name,
            email,
            salted_password: transmuted_salted_password,
            salt: transmuted_salt,
            nick_name,
            color,
            nationality,
            time_zone,
            location,
            locale,
            generation: state.generation + 1,
          }
        }
      }
    };
    Ok(data)
  }

  fn handle_command(state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>, Error> {
    match (state, &cmd) {
      (PlayerData {generation: 0, ..}, PlayerCommand::Create {..}) => Ok(vec![cmd.into()]),
      (_, PlayerCommand::Create {..}) => Err(Error{kind: CommandFailure("Create is only valid on generation 0".to_string())})
    }
  }
}
// FIXME functions below should be in some kind of trait with a standard implementation for
// all events we build
pub fn play_player(events: Vec<PlayerEvent>) -> PlayerData {
  let initial_state: PlayerData = PlayerData {
    id: Uuid::nil(),
    real_name: "".to_string(),
    email: "".to_string(),
    salted_password: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    salt: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nick_name: "".to_string(),
    color: PlayerColor { r:0, g:0, b:0 },
    nationality: None,
    time_zone: None,
    location: None,
    locale: "".to_string(),
    generation: 0,
  };
  apply_events(initial_state, events)
}

fn apply_events(initial_state: PlayerData, events: Vec<PlayerEvent>) -> PlayerData {
  events.into_iter().fold(initial_state, |state, evt| {
    Player::apply_event(&state, evt).unwrap()
  })
}

pub fn add_command(state: PlayerData, cmd: PlayerCommand) -> Result<PlayerData, Error> {
  let events: &Vec<PlayerEvent> = &Player::handle_command(&state, cmd).unwrap();
  let db = db::establish_connection();
  for evt in events.into_iter() {
    // FIXME error handling
    db::insert_event(&db, db::MongoEventCollection::Player, evt).unwrap();
  }
  Ok(apply_events(state, events.to_vec()))
}

#[derive(Deserialize)]
struct PlayerAddData {
    real_name: String,
    email: String,
    password: String,
    nick_name: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    time_zone: Option<i8>,
    location: Option<Country>,
    locale: Locale,
}

#[post("/add", format = "json", data = "<player>")]
fn add_player(player: Json<PlayerAddData>) -> () {
  // FIXME ugh. Really need to modify event sourcing library to use an option instead
  let initial_state: PlayerData = PlayerData {
    id: Uuid::nil(),
    real_name: "".to_string(),
    email: "".to_string(),
    salted_password: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    salt: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    nick_name: "".to_string(),
    color: PlayerColor { r:0, g:0, b:0 },
    nationality: None,
    time_zone: None,
    location: None,
    locale: "".to_string(),
    generation: 0,
  };
  // FIXME wrong function to use. You should initialize all players via play_player() instead, and then
  // input that state whenever add_event is used
  add_command(initial_state, PlayerCommand::Create {
    real_name: player.real_name.clone(),
    email: player.email.clone(),
    password: player.password.clone(),
    nick_name: player.nick_name.clone(),
    color: player.color.clone(),
    nationality: player.nationality.clone(),
    time_zone: player.time_zone,
    location: player.location.clone(),
    locale: player.locale.clone(),
  }).unwrap();
  ()
}

pub fn routes() -> Vec<Route> {
  routes![add_player]
}
