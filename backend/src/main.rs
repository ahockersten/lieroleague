#![feature(proc_macro_hygiene, decl_macro)]

use rocket;

use self::lieroleague::*;

#[get("/")]
fn index() -> () {

}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
