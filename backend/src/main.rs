#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate lieroleague;
use mongodb::db::Database;
use lieroleague::player;
use lieroleague::state;
use lieroleague::db;

#[get("/")]
fn index() -> () {

}


fn main() {
  rocket::ignite()
    .attach(db::LieroLeagueDb::fairing())
    .manage(state::State { player_data: vec!{}})
    .mount("/", routes![index])
    .mount("/player", player::routes())
    .launch();
}
