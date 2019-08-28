#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate lieroleague;
use self::lieroleague::*;

#[get("/")]
fn index() -> () {
    print!("{}", db::test_db())
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
