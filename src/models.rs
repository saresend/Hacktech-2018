
use geo::Point;

use schema::*;

// This is the payload 
#[derive(Serialize, Deserialize)]
pub struct Payload {
    url: String, 
    name: String, 
    location: Point<f64>, 
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Event {
    id: i32,
    name: String, 
    lat: f32, 
    lng: f32,
    pub startTime: String, 
    endTime: String,
   
}


#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[table_name = "CurrEvent"]
pub struct NewEvent {
    name: String, 
    lat: f32, 
    lng: f32, 
    startTime: String, 
    endTime: String, 
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
#[derive(Associations)]
#[belongs_to(Event, foreign_key="id")]
#[table_name = "NewPost"]
pub struct Post {
    id: i32,
    url: String,
    userId: String, 
    eventId: i32,
    lat: f32, 
    lng: f32, 
}

#[derive(Insertable, Serialize)]
#[table_name = "NewPost"]
#[derive(Deserialize)]
pub struct UploadPost {
    pub userId: String, 
    pub eventId: i32,
    pub url: String,
    pub lat: f32, 
    pub lng: f32,
}

