extern crate mongodb;

use crate::player::PlayerEvent;

use chrono::DateTime;
use chrono::Utc;
use enum_display_derive::Display;
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc};
use rocket_contrib::database;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Display)]
pub enum MongoEventCollection {
    Player,
}

#[derive(Serialize, Deserialize)]
struct MongoEvent<T> {
    timestamp: DateTime<Utc>,
    data: T,
}

#[database("lieroleague")]
pub struct LieroLeagueDb(Database);

pub fn insert_event<T: Serialize>(
    db: &Database,
    collection: MongoEventCollection,
    data: T,
) -> Result<(), String> {
    let coll = db.collection(&collection.to_string());
    let event = MongoEvent::<T> {
        timestamp: Utc::now(),
        data: data,
    };
    // FIXME error handling
    let event_bson = bson::to_bson(&event).unwrap();
    match event_bson {
        bson::Bson::Document(event_doc) => {
            coll.insert_one(event_doc, None).unwrap();
            Ok(())
        }
        _ => Err("Oh no".to_string()),
    }
}

pub fn initialize_models(db: &Database) {
    let player_event_map = fetch_player_events(&db);
    ()
}

fn fetch_player_events(db: &Database) -> HashMap<Uuid, Vec<PlayerEvent>> {
    let coll = db.collection(&MongoEventCollection::Player.to_string());
    let player_events_map = HashMap::new();
    let mut cursor = coll
        .aggregate(
            vec![doc! { "$group": {"_id": "$data.Created.id", "events": {"$push": "$$ROOT"}}}],
            None,
        )
        .unwrap();
    let documents = cursor
        .into_iter()
        .collect::<Vec<Result<bson::Document, mongodb::Error>>>();
    for player_events in documents {
        println!("{}", player_events.unwrap().to_string());
    }
    player_events_map
}
