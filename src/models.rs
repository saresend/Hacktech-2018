
use geo::Point;



// This is the payload 
#[derive(Serialize, Deserialize)]
pub struct Payload {
    url: String, 
    name: String, 
    location: Point<f64>, 
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String, 
    lat: f64, 
    lng: f64,
    start_time: String, 
    end_time: String,
    images: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    url: String,
    user_id: i32, 
    lat: f64, 
    lng: f64, 
}
