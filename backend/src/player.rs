use crate::db;
use crate::db::LieroLeagueDb;
use crate::db::MongoEvent;
use crate::state;
use crate::state::State;
use chrono::Utc;
use eventsourcing::Kind::CommandFailure;
use eventsourcing::{Aggregate, AggregateState, Error};
use eventsourcing_derive::Event;
use passwords::hasher;
use rocket::{get, post, routes, Route};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::mem;
use uuid::Uuid;

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

impl From<PlayerEvent> for MongoEvent<PlayerEvent> {
    fn from(evt: PlayerEvent) -> Self {
        match evt {
            PlayerEvent::Created {
                id,
                real_name: _,
                email: _,
                salted_password: _,
                salt: _,
                nick_name: _,
                color: _,
                nationality: _,
                time_zone: _,
                location: _,
                locale: _,
            } => MongoEvent {
                id: id,
                timestamp: Utc::now(),
                data: evt,
            },
        }
    }
}

impl From<PlayerCommand> for PlayerEvent {
    fn from(evt: PlayerCommand) -> Self {
        match evt {
            PlayerCommand::Create {
                real_name,
                email,
                password,
                nick_name,
                color,
                nationality,
                time_zone,
                location,
                locale,
            } => {
                let salt = hasher::gen_salt();
                let salted_password = hasher::bcrypt(12, &salt, &password).unwrap(); // TODO: this could fail
                unsafe {
                    // No way to store unsigned values in BSON, do a conversion here instead
                    let transmuted_salt = mem::transmute::<[u8; 16], [i8; 16]>(salt);
                    let transmuted_salted_password =
                        mem::transmute::<[u8; 24], [i8; 24]>(salted_password);

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
                        locale,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    b: u8,
}

struct Player;

impl Aggregate for Player {
    type Event = PlayerEvent;
    type Command = PlayerCommand;
    type State = PlayerData;

    fn apply_event(state: &Option<Self::State>, evt: Self::Event) -> Result<Self::State, Error> {
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
                locale,
            } => {
                unsafe {
                    // These were stored signed in the database (see above),
                    // so transmute them back here
                    let transmuted_salt = mem::transmute::<[i8; 16], [u8; 16]>(salt);
                    let transmuted_salted_password =
                        mem::transmute::<[i8; 24], [u8; 24]>(salted_password);
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
                        generation: 0,
                    }
                }
            }
        };
        Ok(data)
    }

    fn handle_command(
        state: &Option<Self::State>,
        cmd: Self::Command,
    ) -> Result<Vec<Self::Event>, Error> {
        match (state, &cmd) {
            (None, PlayerCommand::Create { .. }) => Ok(vec![cmd.into()]),
            (_, PlayerCommand::Create { .. }) => Err(Error {
                kind: CommandFailure("Create is only valid on generation 0".to_string()),
            }),
        }
    }
}
// FIXME functions below should be in some kind of trait with a standard implementation for
// all events we build
// Or just use Dispatcher? https://docs.rs/eventsourcing/0.1.1/eventsourcing/trait.Dispatcher.html
pub fn play_player(events: Vec<PlayerEvent>) -> PlayerData {
    apply_events(None, events)
}

fn apply_events(initial_state: Option<PlayerData>, events: Vec<PlayerEvent>) -> PlayerData {
    let mut events_iter = events.into_iter();
    let first_evt = events_iter.next().unwrap();
    let next_state = Player::apply_event(&initial_state, first_evt).unwrap();
    events_iter.fold(next_state, |state, evt| {
        Player::apply_event(&Some(state), evt).unwrap()
    })
}

pub fn add_command(
    db: LieroLeagueDb,
    player_data: Option<PlayerData>,
    cmd: PlayerCommand,
) -> Result<PlayerData, Error> {
    let events: &Vec<PlayerEvent> = &Player::handle_command(&player_data, cmd).unwrap();
    for evt in events.into_iter() {
        // FIXME error handling
        let mongo_evt = MongoEvent::from(evt.clone());
        db::insert_event(&*db, db::MongoEventCollection::Player, &mongo_evt).unwrap();
    }
    Ok(apply_events(player_data, events.to_vec()))
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

#[get("/get", format = "json")]
fn get_player(db: LieroLeagueDb, state: rocket::State<State>) -> Json<Vec<PlayerData>> {
    // FIXME this is needed everywhere right now :/
    state::initialize_state(&db, state.clone());
    let s = state.clone();
    let inner_state = s.lock().unwrap();
    Json(inner_state.player_data.clone())
}

#[post("/add", format = "json", data = "<player>")]
fn add_player(db: LieroLeagueDb, player: Json<PlayerAddData>, state: rocket::State<State>) -> () {
    state::initialize_state(&db, state.clone());
    add_command(
        db,
        None,
        PlayerCommand::Create {
            real_name: player.real_name.clone(),
            email: player.email.clone(),
            password: player.password.clone(),
            nick_name: player.nick_name.clone(),
            color: player.color.clone(),
            nationality: player.nationality.clone(),
            time_zone: player.time_zone,
            location: player.location.clone(),
            locale: player.locale.clone(),
        },
    )
    .unwrap();
    ()
}

pub fn routes() -> Vec<Route> {
    routes![add_player, get_player]
}
