//service/mod.rs

extern crate chrono;
extern crate diesel;
extern crate rocket;
use crate::models;
use crate::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use google_maps::prelude::*;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    lon: f64,
    lat: f64,
}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/simulate", format = "json", data = "<cords>")]
pub fn simulate(cords: Json<Coordinates>) -> Result<Created<Json<Coordinates>>> {
    use self::chrono::prelude::*;
    use models::Presence;
    let connection = &mut establish_connection_pg();

    // let new_cords = Coordinates {
    //     lon: cords.lon,
    //     lat: cords.lat,
    // };

    let presence = Presence {
        id: 1,
        id_str: "1".to_string(),
        msg_type: "connected".to_string(),
        reason: "engine started".to_string(),
        asset: "asset i donno".to_string(),
        time: Utc::now().to_string(),
    };

    diesel::insert_into(self::schema::presences::dsl::presences)
        .values(&presence)
        .execute(connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(cords))
}

#[get("/")]
pub async fn index() -> Template {
    // use self::models::Post;
    // let connection = &mut establish_connection_pg();
    // let results = self::schema::posts::dsl::posts
    //     .load::<Post>(connection)
    //     .expect("Error loading posts");
    // maps().await;
    Template::render("index", context! {})
}

async fn maps() -> () {
    let google_maps_client = GoogleMapsClient::new("AIzaSyAo1agGjrUSZhLwPydiX-_dJ-CEQkxoRmU");
    let directions = google_maps_client
        .directions(
            Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
            Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;

    println!("{:#?}", directions);
}
