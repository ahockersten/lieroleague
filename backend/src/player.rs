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
use rocket::http::hyper::StatusCode;
use rocket::http::hyper::StatusCode::Forbidden;
use rocket::http::{Cookie, Cookies};
use rocket::{get, post, routes, Route};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        time_zone: Option<TimeZoneString>,
        location: Option<Country>,
        locale: Locale,
    },
    LoginSuccess {
        id: Uuid,
    },
    LoginFail {
        id: Uuid,
    }, // FIXME add ip number of login attempt for tracking purposes
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
        time_zone: Option<TimeZoneString>,
        location: Option<Country>,
        locale: Locale,
    },
    LoggedIn {
        id: Uuid,
    },
    LoginFailure {
        id: Uuid,
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
            PlayerEvent::LoggedIn { id } => MongoEvent {
                id: id,
                timestamp: Utc::now(),
                data: evt,
            },
            PlayerEvent::LoginFailure { id } => MongoEvent {
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
            PlayerCommand::LoginSuccess { id } => PlayerEvent::LoggedIn { id },
            PlayerCommand::LoginFail { id } => PlayerEvent::LoginFailure { id },
        }
    }
}

type Email = String;
type Nationality = String;
type Country = String;
type Locale = String;
type TimeZoneString = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerColor {
    #[serde(with = "bson::compat::u2f")]
    r: u8,
    #[serde(with = "bson::compat::u2f")]
    g: u8,
    #[serde(with = "bson::compat::u2f")]
    b: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub id: Uuid,
    pub real_name: String,
    pub email: Email,
    salted_password: [u8; 24],
    salt: [u8; 16],
    pub nick_name: String,
    pub color: PlayerColor,
    pub nationality: Option<Nationality>,
    pub time_zone: Option<TimeZoneString>,
    pub location: Option<Country>,
    pub locale: Locale,
    generation: u64,
}

impl AggregateState for PlayerData {
    fn generation(&self) -> u64 {
        self.generation
    }
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
                    // These were stored signed in the database (see PlayerCommand to PlayerEvent
                    // conversion above), so transmute them back here
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
            PlayerEvent::LoggedIn { .. } => state.clone().unwrap(),
            PlayerEvent::LoginFailure { .. } => state.clone().unwrap(),
        };
        Ok(data)
    }

    fn handle_command(
        state: &Option<Self::State>,
        cmd: Self::Command,
    ) -> Result<Vec<Self::Event>, Error> {
        match (state, &cmd) {
            (None, PlayerCommand::Create { .. }) => Ok(vec![cmd.into()]),
            (None, _) => Err(Error {
                kind: CommandFailure(
                    "Only create is valid when there is no existing data".to_string(),
                ),
            }),
            (_, PlayerCommand::Create { .. }) => Err(Error {
                kind: CommandFailure("Create is only valid on generation 0".to_string()),
            }),
            (_, _) => Ok(vec![cmd.into()]),
        }
    }
}

pub fn initialize_player(events: Vec<PlayerEvent>) -> PlayerData {
    apply_events(None, None, events)
}

fn apply_events(
    initial_state: Option<PlayerData>,
    in_state: Option<rocket::State<State>>,
    events: Vec<PlayerEvent>,
) -> PlayerData {
    let mut events_iter = events.into_iter();
    // calling this function with an empty list and no state makes no sense
    let first_evt = events_iter.next().unwrap();
    let next_state = Player::apply_event(&initial_state, first_evt).unwrap();
    let player_data = events_iter.fold(next_state, |state, evt| {
        Player::apply_event(&Some(state), evt).unwrap()
    });
    if in_state.is_some() {
        let unwrapped_state = in_state.unwrap();
        let mut inner_state = unwrapped_state.lock().unwrap();
        inner_state
            .player_data
            .insert(player_data.id, player_data.clone());
    }
    player_data
}

pub fn add_command(
    db: LieroLeagueDb,
    state: rocket::State<State>,
    player_data: Option<PlayerData>,
    cmd: PlayerCommand,
) -> Result<PlayerData, Error> {
    let events: &Vec<PlayerEvent> = &Player::handle_command(&player_data, cmd).unwrap();
    for evt in events.into_iter() {
        // FIXME error handling
        let mongo_evt = MongoEvent::from(evt.clone());
        db::insert_event(&*db, db::MongoEventCollection::Player, &mongo_evt).unwrap();
    }
    Ok(apply_events(player_data, Some(state), events.to_vec()))
}

trait UserLoginError {}

#[derive(Debug, Clone)]
struct UserNotFoundError;
impl UserLoginError for UserNotFoundError {}
#[derive(Debug, Clone)]
struct UserFailedLoginError;
impl UserLoginError for UserFailedLoginError {}

fn verify_login(
    player_data: PlayerData,
    login_data: PlayerLoginData,
) -> Result<PlayerCommand, UserFailedLoginError> {
    let password_ok_result = hasher::identify_bcrypt(
        12,
        &player_data.salt,
        &login_data.password,
        &player_data.salted_password,
    );
    match password_ok_result {
        Ok(true) => Ok(PlayerCommand::LoginSuccess { id: player_data.id }),
        Ok(false) => Ok(PlayerCommand::LoginFail { id: player_data.id }),
        Err(err) => {
            println!("Login failed due to error {}", err);
            Err(UserFailedLoginError)
        }
    }
}

fn find_existing_player_data_by_email(
    player_datas: HashMap<Uuid, PlayerData>,
    email: String,
) -> Option<PlayerData> {
    player_datas
        .values()
        .find(|&player| player.email == email)
        .map(|player_data| player_data.clone())
}

#[derive(Deserialize)]
struct PlayerAddData {
    real_name: String,
    email: String,
    password: String,
    nick_name: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    time_zone: Option<TimeZoneString>,
    location: Option<Country>,
    locale: Locale,
}

#[post("/add", format = "json", data = "<player>")]
fn add_player(
    db: LieroLeagueDb,
    player: Json<PlayerAddData>,
    state: rocket::State<State>,
) -> Result<(), StatusCode> {
    state::initialize_state(&db, state.clone());
    let s = state.clone();
    let maybe_player_data = find_existing_player_data_by_email(
        s.lock().unwrap().player_data.clone(),
        player.email.clone(),
    );
    match maybe_player_data {
        None => {
            add_command(
                db,
                state,
                None,
                PlayerCommand::Create {
                    real_name: player.real_name.clone(),
                    email: player.email.clone(),
                    password: player.password.clone(),
                    nick_name: player.nick_name.clone(),
                    color: player.color.clone(),
                    nationality: player.nationality.clone(),
                    time_zone: player.time_zone.clone(),
                    location: player.location.clone(),
                    locale: player.locale.clone(),
                },
            )
            .unwrap();
            Ok(())
        }
        Some(_player_data) => Err(Forbidden),
    }
}

#[derive(Deserialize)]
struct PlayerLoginData {
    email: Email,
    password: String,
}

#[post("/login", format = "json", data = "<login_data_json>")]
fn login_player(
    db: LieroLeagueDb,
    login_data_json: Json<PlayerLoginData>,
    state: rocket::State<State>,
    mut cookies: Cookies,
) -> Option<()> {
    state::initialize_state(&db, state.clone());
    let s = state.clone();
    let player_datas = s.lock().unwrap().player_data.clone();
    let login_data = login_data_json.into_inner();
    let maybe_player_data =
        find_existing_player_data_by_email(player_datas, login_data.email.clone());
    match maybe_player_data {
        Some(player_data) => {
            let command_result = verify_login(player_data.clone(), login_data);
            match command_result {
                Ok(command) => match add_command(db, state, Some(player_data), command) {
                    Ok(data) => {
                        cookies.add_private(
                            Cookie::build("user_id", data.id.to_string())
                                .secure(false) // FIXME true if not-dev mode
                                .finish(),
                        );
                        Some(())
                    }
                    Err(err) => {
                        println!("Error occurred during login {:?}", err);
                        None
                    }
                },
                Err(err) => {
                    println!("Error occurred during login {:?}", err);
                    None
                }
            }
        }
        None => None,
    }
}

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct PlayerProfile {
    nickName: String,
    email: Email,
    realName: String,
    color: PlayerColor,
    nationality: Option<Nationality>,
    timeZone: Option<TimeZoneString>,
    location: Option<Country>,
    locale: Locale,
}

#[get("/profile", format = "json")]
fn get_profile(
    db: LieroLeagueDb,
    state: rocket::State<State>,
    mut cookies: Cookies,
) -> Option<Json<PlayerProfile>> {
    // FIXME this is needed everywhere right now :/
    state::initialize_state(&db, state.clone());
    let maybe_user_id_cookie = cookies.get_private("user_id");
    match maybe_user_id_cookie {
        None => None,
        Some(user_id_cookie) => {
            let s = state.clone();
            let inner_state = s.lock().unwrap();
            let cookie_uuid = Uuid::parse_str(user_id_cookie.value()).unwrap();
            let player_data = inner_state
                .player_data
                .values()
                .cloned()
                .find(|player| player.id == cookie_uuid)
                .unwrap();
            Some(Json(PlayerProfile {
                nickName: player_data.nick_name,
                email: player_data.email,
                realName: player_data.real_name,
                color: player_data.color,
                nationality: player_data.nationality,
                timeZone: player_data.time_zone,
                location: player_data.location,
                locale: player_data.locale,
            }))
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![add_player, get_profile, login_player]
}
