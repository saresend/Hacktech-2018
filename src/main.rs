#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate serde;
extern crate geo;
#[macro_use] extern crate serde_derive;

extern crate rocket_cors;
extern crate rocket;

extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

mod models;
mod database;
mod schema; 

use rocket::Data;
use rocket::response::NamedFile;

use std::path::{Path, PathBuf};
use std::iter::Iterator;
use std::cmp::Ordering;
use std::fs::create_dir_all;
use rocket_contrib::Json;
use rand::{thread_rng, Rng};
use schema::*;
use models::*;
use diesel::RunQueryDsl;

use diesel::QueryDsl;
use rocket_cors::{Cors, AllowedOrigins, AllowedHeaders};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[post("/image/<eventId>/<userId>/<lat>/<lng>", format = "image/jpeg", data = "<data>")]
fn upload_image(eventId: i32,userId: String, lat: f32, lng: f32, data: Data) {
   
    let s: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
    create_dir_all(String::from("static/") + &eventId.to_string()).unwrap();
    let path = PathBuf::from("static/").join(eventId.to_string()).join(s.clone() + ".png");
    println!("{:?}", path);
    let url = String::from(String::from("http://f19752fb.ngrok.io/image/")  + &eventId.to_string() + "/" + &s);
    let new_post = UploadPost {  userId, eventId ,url, lat, lng};
    let conn = database::get_database_connection();
    diesel::insert_into(NewPost::table).values(&new_post).execute(&conn).expect("Couldn't add image");
    data.stream_to_file(path).unwrap();
   

}
#[get("/image/<event_id>/<name>")] 
fn get_image(name: String, event_id: String) -> Option<NamedFile> {
    let path = PathBuf::from("static/").join(event_id).join(name + ".png");
    NamedFile::open(path).ok()
}

#[post("/event/add", data = "<event>")]
fn add_event(event: Json<NewEvent>) {
    let conn = database::get_database_connection();
    let data = event.into_inner();
    diesel::insert_into(CurrEvent::table).values(&data).execute(&conn).expect("Failed");
}

#[get("/events/<_lat>/<_lng>")]
fn get_events(_lat: f32, _lng: f32) -> Json<Vec<Event>> {
    let conn = database::get_database_connection();
    let mut results = CurrEvent::table.load::<Event>(&conn).expect("couldn't pull them");
    results.sort_by(|a, b| match a.startTime < b.startTime {
        true => Ordering::Less, 
        _ => Ordering::Greater,
    });
    Json(results)
}

#[get("/photos/<event_id_param>")]
fn get_photos_for_event(event_id_param: i32) -> Json<Vec<Post>> {
    use NewPost::dsl::*;
    use diesel::ExpressionMethods;
    let conn = database::get_database_connection();
    let results = NewPost.filter(eventId.eq(&event_id_param)).load::<Post>(&conn).expect("Couldn't load");

    Json(results)
}

#[get("/events/all")]
fn get_all_events() -> Json<Vec<Event>> {
    let conn = database::get_database_connection();
    let results = CurrEvent::table.load::<Event>(&conn).expect("couldn't pull them");
    Json(results)
}
#[get("/photos/all")]
fn get_all_photos() -> Json<Vec<Post>> {
    let conn = database::get_database_connection();
    let res = schema::NewPost::table.load::<Post>(&conn).expect("Couldn't get data");
    Json(res)
}


fn main() {
    rocket::ignite().mount("/", routes![index,
    upload_image,
    get_image,
    add_event,
    get_events, get_photos_for_event, get_all_events, get_all_photos])
    .attach(rocket_cors::Cors::default()).launch();
}
