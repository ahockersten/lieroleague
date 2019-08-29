#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

//use self::lieroleague::*;

#[get("/")]
fn index() -> () {

}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
