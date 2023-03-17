//service/mod.rs

extern crate chrono;
extern crate rocket;
use crate::models;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;

use rocket::form::Form;
use rocket::FromForm;
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use tokio::time::{self, Duration};
use url::{ParseError, Url};

#[post("/simulate", data = "<user_input>")]
pub fn simulate(user_input: Form<UserInput>) -> Template {
    let new_user_input = UserInput {
        url: user_input.url.to_string(),
        lon: user_input.lon.to_string(),
        lat: user_input.lat.to_string(),
    };

    if ping_server(new_user_input.url.clone()) {
        tokio::spawn(async move {
            use google_maps::prelude::*;

            let google_maps_client =
                GoogleMapsClient::new("AIzaSyAo1agGjrUSZhLwPydiX-_dJ-CEQkxoRmU");
            let directions = google_maps_client
                .directions(
                    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
                    Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
                )
                .with_travel_mode(TravelMode::Driving)
                .execute()
                .await;

            send_presence(directions, &new_user_input.url.clone()).await;
        });
        return Template::render("index", context! {msg:"Simulating !"});
    }
    Template::render(
        "index",
        context! {msg:"Pinging Server but not receiving a pong ! check your URL"},
    )
}

async fn send_presence(
    directions: core::result::Result<
        google_maps::directions::response::Response,
        google_maps::directions::error::Error,
    >,
    url: &String,
) -> () {
    use std::collections::HashMap;
    let mut ten_minutes = time::interval(Duration::from_secs(5));
    loop {
        ten_minutes.tick().await;
        println!("sending ...");
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        map.insert("lang", "rust");
        map.insert("body", "json");
        let res = client.post(url).json(&map).send().await;
        match res {
            Ok(a) => continue,
            Err(err) => break,
        }
    }
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {msg:""})
}

#[get("/store_presence")]
pub async fn store_p() -> Template {
    store_presence().await;
    Template::render("index", context! {msg:"Stored Successfully"})
}

#[get("/store_tracks")]
pub fn store_t() -> Template {
    Template::render("index", context! {msg:""})
}

async fn store_presence() -> Result<(), Box<dyn Error>> {
    use models::Presence;
    use std::fs;

    let data = fs::read_to_string("./presence.json").expect("Unable to read file");

    let json_data: Vec<Presence> = serde_json::from_str(&data).expect("Unable to read file");

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection = client.database("munic").collection("presences");

    for presence in json_data {
        collection.insert_one(presence, None).await;
    }
    Ok(())
}

// fn store_tracks() {
//     // let results = self::schema::posts::dsl::posts
//     //     .load::<Post>(connection)
//     //     .expect("Error loading posts");

//     use models::Presence;
//     use serde_json::Value;
//     use std::fs;

//     let data = fs::read_to_string("./presence.json").expect("Unable to read file");

//     let json_data: serde_json::Value = serde_json::from_str(&data).expect("Unable to read file");

//     let connection = &mut establish_connection_pg();
//     if let Some(pres) = json_data.as_array() {
//         for presence in pres {
//             // Presence {
//             //         id: presence["id"].to_string(),
//             //         id_str: presence["id_str"].to_string(),
//             //         typ: presence["type"].to_string(),
//             //         connection_id: presence["connection_id"].to_string(),
//             //         fullreason: presence["fullreason"].to_string(),
//             //         cs: presence["cs"].to_string(),
//             //         ip: presence["ip"].to_string(),
//             //         protocol: presence["protocol"].to_string(),
//             //         reason: presence["reason"].to_string(),
//             //         asset: presence["asset"].to_string(),
//             //         time: presence["time"].to_string(),
//             //     }
//         }
//     }
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn ping_server(url: String) -> bool {
    use dns_lookup::lookup_host;
    use winping::{Buffer, Pinger};

    let u: Url = Url::parse(&url).unwrap();

    let ips: Vec<std::net::IpAddr> = lookup_host(&u.host().unwrap().to_string()).unwrap();

    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();
    match pinger.send(ips[0], &mut buffer) {
        Ok(_) => true,
        Err(err) => false,
    }
}

#[derive(FromForm)]
pub struct UserInput {
    lon: String,
    lat: String,
    url: String,
}

fn base_url(mut url: Url) -> Result<Url, ParseError> {
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
