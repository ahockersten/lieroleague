#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate lieroleague;
use lieroleague::db;
use lieroleague::player;
use lieroleague::state;
use rocket::http::Method;
use rocket_cors;
use rocket_cors::{AllowedOrigins, Error};

#[get("/")]
fn index() -> () {}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let allowed_methods = vec![Method::Get, Method::Post]
        .into_iter()
        .map(From::from)
        .collect();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    // FIXME you should be able to initialize state here, if you can initialize the db
    // here as well?

    rocket::ignite()
        .attach(db::LieroLeagueDb::fairing())
        .manage(state::State::default())
        .mount("/", routes![index])
        .mount("/player", player::routes())
        .attach(cors)
        .launch();

    Ok(())
}
