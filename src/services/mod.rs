//service/mod.rs

extern crate chrono;
extern crate diesel;
extern crate rocket;
use crate::models;
use crate::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use std::env;
use tokio::time::{self, Duration};

#[post("/simulate", format = "json", data = "<cords>")]
pub fn simulate(cords: Json<Coordinates>) -> Result<Created<Json<Coordinates>>> {
    use self::chrono::prelude::*;
    use models::Presence;
    let connection = &mut establish_connection_pg();

    let new_cords = Coordinates {
        url: cords.url.to_string(),
        lon: cords.lon,
        lat: cords.lat,
    };

    if (ping_server(new_cords.url)) {}
    // let presence = Presence {
    //     id: 1,
    //     id_str: "1".to_string(),
    //     TYPE: "connected".to_string(),
    //     reason: "engine started".to_string(),
    //     asset: "asset i donno".to_string(),
    //     time: Utc::now().to_string(),
    // };

    // diesel::insert_into(self::schema::presences::dsl::presences)
    //     .values(&presence)
    //     .execute(connection)
    //     .expect("Error saving new post");
    Ok(Created::new("/").body(cords))
}

use url::{ParseError, Url};
fn ping_server(url: String) -> bool {
    use dns_lookup::lookup_host;
    use std::net::IpAddr;
    use winping::{Buffer, Pinger};

    let u = Url::parse(&url).unwrap();

    // println!("host {}", u.host().unwrap().to_string());
    let ips: Vec<std::net::IpAddr> = lookup_host(&u.host().unwrap().to_string()).unwrap();

    // println!("ip {}", ips[0]);
    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();
    match pinger.send(ips[0], &mut buffer) {
        Ok(rtt) => true,
        Err(err) => false,
    }
}

fn base_url(mut url: Url) -> Result<Url> {
    use std::io::{Error, ErrorKind};

    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            panic!("Cannot find a base!");
        }
    }

    url.set_query(None);

    Ok(url)
}

#[get("/")]
pub async fn index() -> Template {
    // use self::models::Post;
    // let connection = &mut establish_connection_pg();
    // let results = self::schema::posts::dsl::posts
    //     .load::<Post>(connection)
    //     .expect("Error loading posts");
    // get_directions().await;

    tokio::spawn(async move {
        use google_maps::prelude::*;

        let google_maps_client = GoogleMapsClient::new("AIzaSyAo1agGjrUSZhLwPydiX-_dJ-CEQkxoRmU");
        let directions = google_maps_client
            .directions(
                Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
                Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
            )
            .with_travel_mode(TravelMode::Driving)
            .execute()
            .await;

        send_presence(directions).await;
    });

    Template::render("index", context! {})
}
async fn send_presence(
    directions: core::result::Result<
        google_maps::directions::response::Response,
        google_maps::directions::error::Error,
    >,
) -> () {
    use models::Presence;
    use serde_json;
    use serde_json::Value;
    use std::fs;

    let data = fs::read_to_string("./presence.json").expect("Unable to read file");

    let json_data: serde_json::Value = serde_json::from_str(&data).expect("Unable to read file");

    let mut presences: Vec<Presence> = vec![];

    if let Some(pres) = json_data.as_array() {
        for presence in pres {
            presences.push(Presence {
                id: presence["id"].as_i64().unwrap(),
                id_str: presence["id_str"].to_string(),
                r#type: presence["type"].to_string(),
                connection_id: presence["connection_id"].as_i64().unwrap(),
                fullreason: presence["fullreason"].to_string(),
                cs: presence["cs"].to_string(),
                ip: presence["ip"].to_string(),
                protocol: presence["protocol"].to_string(),
                reason: presence["reason"].to_string(),
                asset: presence["asset"].to_string(),
                time: presence["time"].to_string(),
            });
        }
        println!("{:#?}", presences);
    }

    // let res: Presence = serde_json::from_str(&data).expect("Unable to parse");

    let mut ten_minutes = time::interval(Duration::from_secs(5));
    loop {
        ten_minutes.tick().await;
        println!("heello");
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    url: String,
    lon: f64,
    lat: f64,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
