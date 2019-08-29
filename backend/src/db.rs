extern crate mongodb;

use mongodb::Client;
use mongodb::ThreadedClient;
use mongodb::db::Database;
use mongodb::db::ThreadedDatabase;

fn establish_connection() -> Database {
  // Direct connection to a server. Will not look for other servers in the topology.
  let client = Client::connect("localhost", 27017)
    .expect("Failed to initialize client.");
  client.db("lieroleague")
}

pub fn test_db() -> i64 {
  let db = establish_connection();
  let coll = db.collection("test");
  coll.count(None, None).unwrap()
}
