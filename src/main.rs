#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;

use rocket::Data;
use rocket::response::NamedFile;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[post("/image/<name>", format = "image/jpeg", data = "<data>")]
fn upload_image(name: String, data: Data) {
   
    let path = PathBuf::from("static/").join(name);
    data.stream_to_file(path).unwrap();
}
#[get("/image/<name>")] 
fn get_image(name: String) -> Option<NamedFile> {
    let path = PathBuf::from("static/").join(name);
    NamedFile::open(path).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, upload_image, get_image]).launch();
}
