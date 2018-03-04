#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate serde;
extern crate geo;
#[macro_use] extern crate serde_derive;

extern crate rocket;

extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

mod models;
mod database;

use rocket::Data;
use rocket::response::NamedFile;

use std::path::PathBuf;
use rocket_contrib::Json;

use models::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[post("/image/<event_id>/<lat>/<lng>", format = "image/jpeg", data = "<data>")]
fn upload_image(event_id: String, lat: f64, lng: f64, data: Data) {
   
    let path = PathBuf::from("static/").join(event_id);

    data.stream_to_file(path).unwrap();

}
#[get("/image/<name>")] 
fn get_image(name: String) -> Option<NamedFile> {
    let path = PathBuf::from("static/").join(name);
    NamedFile::open(path).ok()
}

#[post("/event/add", data = "<event>")]
fn add_event(event: Json<Event>) {
    unimplemented!();
}

#[get("/events/<lat>/<lng>")]
fn get_events(lat: f64, lng: f64) -> Json<Vec<Event>> {
    unimplemented!();
}

#[get("/photos/<event_id>")]
fn get_photos_for_event(event_id: i32) -> Json<Vec<Post>> {
    unimplemented!();
}


fn main() {
    rocket::ignite().mount("/", routes![index, upload_image, get_image, add_event, get_events, get_photos_for_event]).launch();
}
