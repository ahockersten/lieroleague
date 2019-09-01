#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate lieroleague;
use lieroleague::player;

#[get("/")]
fn index() -> () {

}

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .mount("/player", player::routes())
    .launch();
}
