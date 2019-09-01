extern crate mongodb;

use chrono::Utc;
use chrono::DateTime;
use std::fmt::Display;

use mongodb::Client;
use mongodb::ThreadedClient;
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;
use enum_display_derive::Display;
use serde::{Serialize, Deserialize};
use bson;

#[derive(Display)]
pub enum MongoEventCollection {
  Player,
}

#[derive(Serialize, Deserialize)]
struct MongoEvent<T> {
  timestamp: DateTime<Utc>,
  data: T
}

pub fn establish_connection() -> Database {
  // Direct connection to a server. Will not look for other servers in the topology.
  let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize client.");
  client.db("lieroleague")
}

pub fn insert_event<T: Serialize>(db: &Database, collection: MongoEventCollection, data: T) -> Result<(), String> {
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
    _ => Err("Oh no".to_string())
  }
}
