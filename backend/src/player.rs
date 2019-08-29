use serde::{Serialize, Deserialize};
use eventsourcing::{Aggregate, AggregateState, Error};
use eventsourcing_derive::Event;
use uuid::Uuid;
use passwords::hasher;

#[derive(Debug)]
enum PlayerCommand {
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
enum PlayerEvent {
  Created {
    id: Uuid,
    real_name: String,
    email: Email,
    password: [u8; 24],
    salt: [u8; 16],
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
        let salted_password = hasher::bcrypt(12, &salt, &password);
        PlayerEvent::Created {
          id: Uuid::new_v4(),
          real_name,
          email,
          password: salted_password.unwrap(), // TODO: this could fail
          salt: salt,
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

type Email = String;
type Nationality = String;
type Country = String;
type Locale = String;

#[derive(Debug)]
struct PlayerData {
  id: Uuid,
  real_name: String,
  email: Email,
  password: [u8; 24],
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
struct PlayerColor {
  r: u8,
  g: u8,
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
        password,
        salt,
        nick_name,
        color,
        nationality,
        time_zone,
        location,
        locale
      } => PlayerData {
        id,
        real_name,
        email,
        password,
        salt,
        nick_name,
        color,
        nationality,
        time_zone,
        location,
        locale,
        generation: state.generation + 1,
      },
    };
    Ok(data)
  }

  fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>, Error> {
        // SHOULD DO: validate state and command

        // if validation passes...
        Ok(vec![cmd.into()])
      }
    }
