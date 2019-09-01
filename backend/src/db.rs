extern crate mongodb;

use crate::player::PlayerEvent;

use uuid::Uuid;
use chrono::Utc;
use chrono::DateTime;
use std::fmt::Display;
use std::collections::HashMap;
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use enum_display_derive::Display;
use serde::{Serialize, Deserialize};
use bson;
use rocket_contrib::database;

#[derive(Display)]
pub enum MongoEventCollection {
  Player,
}

#[derive(Serialize, Deserialize)]
struct MongoEvent<T> {
  timestamp: DateTime<Utc>,
  data: T
}

#[database("lieroleague")]
pub struct LieroLeagueDb(Database);

pub fn insert_event<T: Serialize>(db: &Database, collection: MongoEventCollection, data: T) -> Result<(), String> {
  let coll = db.collection(&collection.to_string());
  let event = MongoEvent::<T> {
    timestamp: Utc::now(),
    data: data,
  };
  println!("insert_event");
  // FIXME error handling
  let event_bson = bson::to_bson(&event).unwrap();
  match event_bson {
    bson::Bson::Document(event_doc) => {
      println!("insert evn2");
      coll.insert_one(event_doc, None).unwrap();
      Ok(())
    }
    _ => Err("Oh no".to_string())
  }
}

pub fn initialize_models(db: &Database) {
  fetch_player_events(&db);
  ()
}

fn fetch_player_events(db: &Database) -> HashMap<Uuid, Vec<PlayerEvent>> {
  let coll = db.collection(&MongoEventCollection::Player.to_string());
  HashMap::new()
 }
