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
pub enum MatchCommand {
    RegisterMatchPlayed {
        players: Vec<Uuid>,
        type: MatchType,
        mod: Mod,
        game_mode: GameMode
    },
}

enum GameMode {
    KillEmAll {
        lives:
    },
    GameOfTag,
    Holdazone,


}

enum Mod {
    Liero1_33,
    LieroPromodeFinal,
    Csliero22b
}

enum MatchType {
    Friendly,
    Ladder
}
