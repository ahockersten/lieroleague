extern crate mongodb;

use crate::player;
use crate::player::PlayerEvent;
use crate::state::InnerState;
use chrono::DateTime;
use chrono::Utc;
use enum_display_derive::Display;
use eventsourcing::Event;
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
pub struct MongoEvent<T> {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: T,
}

#[database("lieroleague")]
pub struct LieroLeagueDb(Database);

pub fn insert_event<E: Event>(
    db: &Database,
    collection: MongoEventCollection,
    data: &MongoEvent<E>,
) -> Result<(), String> {
    let coll = db.collection(&collection.to_string());
    // FIXME error handling
    let event_bson = bson::to_bson(data).unwrap();
    match event_bson {
        bson::Bson::Document(event_doc) => {
            coll.insert_one(event_doc, None).unwrap();
            Ok(())
        }
        _ => Err("Oh no".to_string()),
    }
}

pub fn initialize_player_data(db: &Database) -> Vec<player::PlayerData> {
    let player_event_map = fetch_player_events(&db);
    println!("{:?}", player_event_map);
    player_event_map
        .values()
        .map(|events| player::play_player(events.to_vec()))
        .collect()
}

fn fetch_events_by_id_sorted_by_timestamp<'a, E: Event + Deserialize<'a>>(
    db: &Database,
    collection: MongoEventCollection,
) -> HashMap<Uuid, Vec<E>> {
    let coll = db.collection(&collection.to_string());
    let mut events_map = HashMap::new();
    let cursor = coll
        .aggregate(
            vec![
              doc! { "$group": {"_id": "$id", "timestamp": {"$push": "$timestamp"}, "events": {"$push": "$data"}}},
              doc! { "$sort": {"timestamp": 1}},
            ],
            None,
        )
        .unwrap();
    let documents = cursor
        .into_iter()
        .collect::<Vec<Result<bson::Document, mongodb::Error>>>();
    for document in documents {
        let unwrapped_doc: bson::Document = document.unwrap();
        let id = bson::from_bson(unwrapped_doc.get("_id").unwrap().clone()).unwrap();
        let events = bson::from_bson(unwrapped_doc.get("events").unwrap().clone()).unwrap();
        events_map.insert(id, events);
    }
    events_map
}

fn fetch_player_events(db: &Database) -> HashMap<Uuid, Vec<PlayerEvent>> {
    fetch_events_by_id_sorted_by_timestamp(db, MongoEventCollection::Player)
}
