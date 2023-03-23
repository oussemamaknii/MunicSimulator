//service/mod.rs

extern crate chrono;
extern crate rocket;
use crate::models::{self, Tracks};
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};

use models::{event, req, Fields};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Cursor,
};
use serde::Deserialize;
use std::env;
use std::error::Error;

use rocket::FromForm;
use rocket::{form::Form, http::ext::IntoCollection};
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use tokio::time::{self, Duration};
use url::{ParseError, Url};

use core::sync::atomic::{AtomicUsize, Ordering};

static state: AtomicUsize = AtomicUsize::new(0);

pub fn set_value(val: usize) {
    state.store(val, Ordering::Relaxed)
}

pub fn get_value() -> usize {
    state.load(Ordering::Relaxed)
}

#[post("/simulate", data = "<user_input>")]
pub fn simulate(user_input: Form<UserInput>) -> Template {
    let new_user_input = UserInput {
        url: user_input.url.to_string(),
        lon: user_input.lon.to_string(),
        lat: user_input.lat.to_string(),
    };

    if ping_server(new_user_input.url.clone()) {
        tokio::spawn(async move {
            use futures::future::join_all;
            let mut handles = vec![];

            let worker = tokio::spawn(async move {
                use google_maps::prelude::*;

                let google_maps_client =
                    GoogleMapsClient::new("AIzaSyAo1agGjrUSZhLwPydiX-_dJ-CEQkxoRmU");
                let directions = google_maps_client
                    .directions(
                        Location::Address(String::from(new_user_input.lat)),
                        Location::Address(String::from(new_user_input.lon)),
                        // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
                    )
                    .with_travel_mode(TravelMode::Driving)
                    .execute()
                    .await;

                send_presence(directions, &new_user_input.url.clone()).await;
                println!("Work Done !")
            });

            handles.push(worker);
            join_all(handles).await;
            println!("Shuting down the Handler thread!!")
        });

        return Template::render("index", context! {msg:"Simulating !"});
    }
    Template::render("index", context! {msg:"Ping missing pong ! check your URL"})
}

async fn send_presence(
    directions: core::result::Result<
        google_maps::directions::response::Response,
        google_maps::directions::error::Error,
    >,
    url: &String,
) -> () {
    use google_maps::prelude::*;
    use polyline;

    let json_data = &directions.unwrap().routes[0].legs[0];

    let durations = &json_data.duration.text;
    let distance = &json_data.distance.text;

    use dotenvy::dotenv;
    use mongodb::bson::doc;
    use mongodb::options::ClientOptions;
    use tokio_stream::StreamExt;
    dotenv().ok();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = mongodb::Client::with_options(options).unwrap();

    let tracks_collection = client.database("munic").collection::<Tracks>("tracks");

    // counting how many records
    let mut number_of_records_needed = 0;

    for step in &json_data.steps {
        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();

        for coord in &result {
            number_of_records_needed = number_of_records_needed + 1;
        }
    }

    let pipeline = vec![
        doc! {
            "$match": {
                "$or":[{"fields.gps_dir":  {"$ne": null}
                         },{
                           "fields.gps_altitude":  {"$ne": null}
                         },{
                           "fields.gps_hdop":  {"$ne": null}
                         },{
                           "fields.gps_pdop":  {"$ne": null}
                         },{
                           "fields.gps_vdop":  {"$ne": null}
                         }, {
                           "fields.gps_average_pdop_status":  {"$ne": null}
                         },{
                           "fields.gps_speed":  {"$ne": null}
                         }] }
        },
        doc! { "$sample": { "size": number_of_records_needed } },
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$sort": { "date" : 1 }},
    ];

    let data = tracks_collection
        .aggregate(pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use bson::{bson, Bson, Document};
    use futures::stream::TryStreamExt;

    let tracks: Vec<_> = data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let mut index = 0;

    'outer: for step in &json_data.steps {
        use substring::Substring;
        use tokio::time::Duration;

        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();
        let mut count = 0;

        for coord in &result {
            count = count + 1;
        }

        let c: char = ' ';
        let mut ten_minutes = time::interval(
            Duration::from_secs_f64(
                (((step.duration.text)
                    .to_string()
                    .substring(0, (step.duration.text).find(' ').unwrap())
                    .parse::<u64>()
                    .unwrap())
                    * 60) as f64
                    / count as f64,
            )
            .try_into()
            .unwrap(),
        );

        for coord in &result {
            ten_minutes.tick().await;
            println!("sending ...");
            let client = reqwest::Client::new();

            use chrono::prelude::*;
            use chrono::Duration;
            use chrono::{DateTime, Local, TimeZone};

            let trk = Tracks {
                id: tracks[index].get_i64("id").unwrap() as i64,
                id_str: Some(tracks[index].get_str("id_str").unwrap().to_string()),
                location: Some([coord.x, coord.y]),
                loc: Some([coord.x, coord.y]),
                asset: Some(tracks[index].get_str("asset").unwrap().to_string()),
                recorded_at: Some(
                    (Local::now() - Duration::minutes(2))
                        .format("%Y-%m-%dT%H:%M:%SZ")
                        .to_string(),
                ),
                recorded_at_ms: Some(
                    (Local::now() - Duration::minutes(2))
                        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                        .to_string(),
                ),
                received_at: Some((Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                connection_id: tracks[index].get_i64("connection_id").unwrap() as i64,
                index: tracks[index].get_i64("index").unwrap() as i64,
                fields: Some(Fields::from(tracks[index].get_document("fields").unwrap())),
                url: Some(tracks[index].get_str("url").unwrap().to_string()),
            };

            let x: req<Tracks> = req {
                meta: event {
                    event: "track".to_string(),
                    account: "municio".to_string(),
                },
                payload: trk,
            };

            let res = client.post(url).json(&x).send().await;
            index = index + 1;
            match res {
                Ok(a) => {
                    set_value(1);
                    continue;
                }
                Err(err) => {
                    set_value(0);
                    break 'outer;
                }
            }
        }
    }
}

#[get("/test")]
pub async fn test() -> () {
    use google_maps::prelude::*;
    use polyline;

    let google_maps_client = GoogleMapsClient::new("AIzaSyAo1agGjrUSZhLwPydiX-_dJ-CEQkxoRmU");
    let directions = google_maps_client
        .directions(
            Location::Address(String::from("ariana, tunisie")),
            Location::Address(String::from("marsa tunisie")),
            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
        )
        .with_travel_mode(TravelMode::Driving)
        .execute()
        .await;

    let json_data = &directions.unwrap().routes[0].legs[0];

    let durations = &json_data.duration.text;
    let distance = &json_data.distance.text;

    use dotenvy::dotenv;
    use mongodb::bson::doc;
    use mongodb::options::ClientOptions;
    use tokio_stream::StreamExt;
    dotenv().ok();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = mongodb::Client::with_options(options).unwrap();

    let tracks_collection = client.database("munic").collection::<Tracks>("tracks");

    // counting how many records
    let mut number_of_records_needed = 0;

    for step in &json_data.steps {
        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();

        for coord in &result {
            number_of_records_needed = number_of_records_needed + 1;
        }
    }

    let pipeline = vec![
        doc! {
            "$match": {
                "$or":[{"fields.gps_dir":  {"$ne": null}
                         },{
                           "fields.gps_altitude":  {"$ne": null}
                         },{
                           "fields.gps_hdop":  {"$ne": null}
                         },{
                           "fields.gps_pdop":  {"$ne": null}
                         },{
                           "fields.gps_vdop":  {"$ne": null}
                         }, {
                           "fields.gps_average_pdop_status":  {"$ne": null}
                         },{
                           "fields.gps_speed":  {"$ne": null}
                         }] }
        },
        doc! { "$sample": { "size": number_of_records_needed } },
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$sort": { "date" : 1 }},
    ];

    let data = tracks_collection
        .aggregate(pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use bson::{bson, Bson, Document};
    use futures::stream::TryStreamExt;

    let tracks: Vec<_> = data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let mut index = 0;

    // match data {
    //     Ok(mut cursor) => {
    //         while let Some(doc) = cursor.next().await {
    //             println!("{:#?}", doc.unwrap())
    //         }
    //     }
    //     Err(e) => println!("{:#?}", e),
    // };

    'outer: for step in &json_data.steps {
        use substring::Substring;
        use tokio::time::Duration;

        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();
        let mut count = 0;

        for coord in &result {
            count = count + 1;
        }

        let c: char = ' ';
        let mut ten_minutes = time::interval(
            Duration::from_secs_f64(
                (((step.duration.text)
                    .to_string()
                    .substring(0, (step.duration.text).find(' ').unwrap())
                    .parse::<u64>()
                    .unwrap())
                    * 60) as f64
                    / count as f64,
            )
            .try_into()
            .unwrap(),
        );

        for coord in &result {
            ten_minutes.tick().await;
            println!("sending ...");
            let client = reqwest::Client::new();

            use chrono::prelude::*;
            use chrono::Duration;
            use chrono::{DateTime, Local, TimeZone};

            let trk = Tracks {
                id: tracks[index].get_i64("id").unwrap() as i64,
                id_str: Some(tracks[index].get_str("id_str").unwrap().to_string()),
                location: Some([coord.x, coord.y]),
                loc: Some([coord.x, coord.y]),
                asset: Some(tracks[index].get_str("asset").unwrap().to_string()),
                recorded_at: Some(
                    (Local::now() - Duration::minutes(2))
                        .format("%Y-%m-%dT%H:%M:%SZ")
                        .to_string(),
                ),
                recorded_at_ms: Some(
                    (Local::now() - Duration::minutes(2))
                        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                        .to_string(),
                ),
                received_at: Some((Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                connection_id: tracks[index].get_i64("connection_id").unwrap() as i64,
                index: tracks[index].get_i64("index").unwrap() as i64,
                fields: Some(Fields::from(tracks[index].get_document("fields").unwrap())),
                url: Some(tracks[index].get_str("url").unwrap().to_string()),
            };

            let x: Vec<req<Tracks>> = vec![req {
                meta: event {
                    account: "municio".to_string(),
                    event: "track".to_string(),
                },
                payload: trk,
            }];

            let res = client
                .post("http://localhost:5000/simulate")
                .json(&x)
                .send()
                .await;
            index = index + 1;
            match res {
                Ok(a) => {
                    set_value(1);
                    continue;
                }
                Err(err) => {
                    set_value(0);
                    break 'outer;
                }
            }
        }
    }
}

#[get("/events")]
pub fn stream() -> EventStream![] {
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            yield Event::data(get_value().to_string());
            interval.tick().await;
        }
    }
}

pub async fn test2() -> () {
    use dotenvy::dotenv;
    use mongodb::bson::doc;
    use mongodb::options::ClientOptions;
    use tokio_stream::StreamExt;
    dotenv().ok();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = mongodb::Client::with_options(options).unwrap();

    let tracks = client.database("munic").collection::<Tracks>("tracks");

    let pipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$sort": { "date" : 1 }},
    ];

    let data = tracks
        .aggregate(pipeline, None)
        .await
        .map_err(|e| println!("{}", e));

    match data {
        Ok(mut cursor) => {
            while let Some(doc) = cursor.next().await {
                println!("{:#?}", doc.unwrap())
            }
        }
        Err(e) => println!("{:#?}", e),
    };
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {msg:""})
}

#[get("/store_presence")]
pub async fn store_p() -> Template {
    tokio::spawn(async move {
        store_presence().await;
    });
    Template::render("index", context! {msg:"Presences Stored Successfully"})
}

#[get("/store_tracks")]
pub async fn store_t() -> Template {
    tokio::spawn(async move { store_tracks().await });
    Template::render("index", context! {msg:"Tracks Stored Successfully"})
}

async fn store_presence() -> Result<(), Box<dyn Error + Send + Sync>> {
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

async fn store_tracks() -> Result<(), Box<dyn Error + Send + Sync>> {
    use dotenvy::dotenv;
    dotenv().ok();
    use std::fs;

    let data = fs::read_to_string("./tracks.json").expect("Unable to read file");

    let json_data: Vec<Tracks> = serde_json::from_str(&data).expect("Unable to read file");

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection = client.database("munic").collection("tracks");

    for track in json_data {
        collection.insert_one(track, None).await;
    }
    Ok(())
}

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
