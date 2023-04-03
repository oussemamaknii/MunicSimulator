//service/mod.rs

extern crate chrono;
extern crate rocket;
use crate::models::{self, Presence, Tracks};
use bson::doc;
use mongodb::Collection;
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::uri;
use std::fs;

use rocket::http::ContentType;
use rocket::Data;

use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use models::{Eveent, Fields, Req};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;
use std::path::PathBuf;

use rocket::form::Form;
use rocket::FromForm;
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use tokio::time::{self, Duration};
use url::Url;

use core::sync::atomic::{AtomicUsize, Ordering};

static STATE: AtomicUsize = AtomicUsize::new(3);

pub fn set_value(val: usize) {
    STATE.store(val, Ordering::Relaxed)
}

pub fn get_value() -> usize {
    STATE.load(Ordering::Relaxed)
}

#[post("/upload", data = "<data>")]
pub async fn file_json(content_type: &ContentType, data: Data<'_>) -> () {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("track_json_file")
            .content_type_by_string(Some(mime::APPLICATION_JSON))
            .unwrap(),
        MultipartFormDataField::file("presence_json_file")
            .content_type_by_string(Some(mime::APPLICATION_JSON))
            .unwrap(),
        // MultipartFormDataField::text("url"),
        // MultipartFormDataField::text("lon"),
        // MultipartFormDataField::text("lat"),
        // MultipartFormDataField::text("track_option"),
        // MultipartFormDataField::text("presence_option"),
    ]);

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    let track_json_file = multipart_form_data.files.get("track_json_file");
    let presence_json_file = multipart_form_data.files.get("presence_json_file");
    // let url_text = multipart_form_data.texts.remove("url");
    // let lon_text = multipart_form_data.texts.remove("lon");
    // let lat_text = multipart_form_data.texts.remove("lat");
    // let track_option = multipart_form_data.texts.remove("track_option");
    // let presence_option = multipart_form_data.texts.remove("presence_option");

    if let Some(tfile_fields) = track_json_file {
        let file_field = &tfile_fields[0];

        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;

        // let curr = env::current_dir().unwrap();
        let mut path = PathBuf::new();

        // path.push(curr);
        path.push("C:\\Users\\makni_o\\\\Documents\\MunicSimulator\\uploads\\");
        match _file_name {
            Some(name) => path.push(name),
            None => return (),
        }

        match fs::rename(_path, path) {
            Ok(_c) => (),
            Err(_e) => panic!("rename panic !"),
        }
        match store_tracks(_file_name).await {
            Ok(_ok) => (),
            Err(_err) => panic!("store panic !"),
        };
        match update_tracks(_file_name).await {
            Ok(_ok) => (),
            Err(_err) => panic!("store panic !"),
        };
    }

    if let Some(pfile_fields) = presence_json_file {
        let file_field = &pfile_fields[0];

        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;

        // let curr = env::current_dir().unwrap();
        let mut path = PathBuf::new();

        // path.push(curr);
        path.push("C:\\Users\\makni_o\\\\Documents\\MunicSimulator\\uploads\\");
        match _file_name {
            Some(name) => path.push(name),
            None => return (),
        }

        match fs::rename(_path, path) {
            Ok(_c) => (),
            Err(_e) => panic!("rename panic !"),
        }
        match store_presence(_file_name).await {
            Ok(_ok) => (),
            Err(_err) => panic!("store panic !"),
        };
        match update_presence(_file_name).await {
            Ok(_ok) => (),
            Err(_err) => panic!("store panic !"),
        };
    }
}

#[post("/simulate", data = "<user_input>")]
pub fn simulate(user_input: Form<UserInput>) -> Redirect {
    let mut new_user_input = UserInput {
        url: user_input.url.to_string(),
        lon: user_input.lon.to_string(),
        lat: user_input.lat.to_string(),
        track_option: user_input.track_option.to_string(),
        presence_option: user_input.presence_option.to_string(),
    };

    if new_user_input.track_option.len() != 10 {
        if new_user_input.track_option.match_indices("-").nth(1) == Some((6, "-")) {
            new_user_input.track_option.insert(5, '0');
        }
        if new_user_input.track_option.match_indices("-").nth(1) == Some((7, "-"))
            && new_user_input.track_option.len() != 10
        {
            new_user_input.track_option.insert(8, '0');
        }
    }

    if new_user_input.presence_option.len() != 10 {
        if new_user_input.presence_option.match_indices("-").nth(1) == Some((6, "-")) {
            new_user_input.presence_option.insert(5, '0');
        }
        if new_user_input.presence_option.match_indices("-").nth(1) == Some((7, "-"))
            && new_user_input.presence_option.len() != 10
        {
            new_user_input.presence_option.insert(8, '0');
        }
    }

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

                set_value(1);
                send_tracks(
                    directions,
                    &new_user_input.url.clone(),
                    &new_user_input.track_option,
                )
                .await;
                println!("Work Done !")
            });

            handles.push(worker);
            join_all(handles).await;
            println!("Shuting down the Handler thread!!")
        });

        return Redirect::to(uri!(index("Sending !")));
    }
    Redirect::to(uri!(index("Didn't receive a Pong !")))
}

async fn send_tracks(
    directions: core::result::Result<
        google_maps::directions::response::Response,
        google_maps::directions::error::Error,
    >,
    url: &String,
    option: &String,
) -> () {
    use google_maps::prelude::*;

    let json_data = &directions.unwrap().routes[0].legs[0];

    use dotenvy::dotenv;
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

        for _ in &result {
            number_of_records_needed = number_of_records_needed + 1;
        }
    }
    let pipeline = vec![
        doc! {"$match": { "fields": { "$gt": {} } }  },
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$addFields": { "subs": {"$substr": [ "$recorded_at", 0, 10 ]} }},
        doc! {
            "$match": { "subs" :  {"$eq": option} }
        },
        doc! { "$sample": { "size": number_of_records_needed } },
        doc! {"$sort": { "date" : 1 }},
    ];

    let data = tracks_collection
        .aggregate(pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use futures::stream::TryStreamExt;

    let tracks: Vec<_> = data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let mut index = 0;

    let client = reqwest::Client::new();

    let mut tracks_array: Vec<Req<Tracks>> = vec![];

    'outer: for step in &json_data.steps {
        use substring::Substring;
        use tokio::time::Duration;

        let result = polyline::decode_polyline(&step.polyline.points, 6).unwrap();

        let mut ten_minutes = time::interval(
            Duration::from_secs_f64(
                (((step.duration.text)
                    .to_string()
                    .substring(0, (step.duration.text).find(' ').unwrap())
                    .parse::<u64>()
                    .unwrap())
                    * 60) as f64
                    / result.num_coords() as f64,
            )
            .try_into()
            .unwrap(),
        );

        for coord in &result {
            ten_minutes.tick().await;
            println!("sending ...");

            use chrono::Duration;

            index = index + 1;

            if tracks_array.len() == 10 {
                break 'outer;
            }

            let builder = {
                if get_value() == 1 {
                    client.post(url).json(&vec![Req {
                        meta: Eveent {
                            event: "track".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Tracks {
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
                            received_at: Some(
                                (Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                            ),
                            connection_id: tracks[index].get_i64("connection_id").unwrap() as i64,
                            index: tracks[index].get_i64("index").unwrap() as i64,
                            fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                            url: Some(tracks[index].get_str("url").unwrap().to_string()),
                        },
                    }])
                } else {
                    client.post(url).json(&tracks_array)
                }
            };
            let res = builder.send().await;

            match res {
                Ok(_a) => match get_value() {
                    1 => continue,
                    0 => tracks_array = vec![],
                    _ => continue,
                },
                Err(_err) => match get_value() {
                    1 => {
                        tracks_array.push(Req {
                            meta: Eveent {
                                event: "track".to_string(),
                                account: "municio".to_string(),
                            },
                            payload: Tracks {
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
                                received_at: Some(
                                    (Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                                ),
                                connection_id: tracks[index].get_i64("connection_id").unwrap()
                                    as i64,
                                index: tracks[index].get_i64("index").unwrap() as i64,
                                fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                                url: Some(tracks[index].get_str("url").unwrap().to_string()),
                            },
                        });
                        set_value(0);
                    }
                    0 => tracks_array.push(Req {
                        meta: Eveent {
                            event: "track".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Tracks {
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
                            received_at: Some(
                                (Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                            ),
                            connection_id: tracks[index].get_i64("connection_id").unwrap() as i64,
                            index: tracks[index].get_i64("index").unwrap() as i64,
                            fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                            url: Some(tracks[index].get_str("url").unwrap().to_string()),
                        },
                    }),
                    _ => continue,
                },
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

    set_value(1);

    let json_data = &directions.unwrap().routes[0].legs[0];

    use dotenvy::dotenv;
    use mongodb::bson::doc;
    use mongodb::options::ClientOptions;
    dotenv().ok();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = mongodb::Client::with_options(options).unwrap();

    let tracks_collection = client.database("munic").collection::<Tracks>("tracks");

    // counting records
    let mut number_of_records_needed = 0;

    for step in &json_data.steps {
        let result = polyline::decode_polyline(&step.polyline.points, 6).unwrap();

        for _ in &result {
            number_of_records_needed = number_of_records_needed + 1;
        }
    }

    let pipeline = vec![
        doc! {"$match": { "fields": { "$ne": {} } }  },
        doc! { "$sample": { "size": number_of_records_needed } },
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$sort": { "date" : 1 }},
    ];

    let data = tracks_collection
        .aggregate(pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use futures::stream::TryStreamExt;

    let tracks: Vec<_> = data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let mut index = 0;

    let client = reqwest::Client::new();

    let mut tracks_array: Vec<Req<Tracks>> = vec![];

    'outer: for step in &json_data.steps {
        use substring::Substring;
        use tokio::time::Duration;

        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();

        let mut ten_minutes = time::interval(
            Duration::from_secs_f64(
                (((step.duration.text)
                    .to_string()
                    .substring(0, (step.duration.text).find(' ').unwrap())
                    .parse::<u64>()
                    .unwrap())
                    * 60) as f64
                    / result.num_coords() as f64,
            )
            .try_into()
            .unwrap(),
        );

        for coord in &result {
            ten_minutes.tick().await;
            println!("sending ...");

            use chrono::Duration;
            use chrono::Local;

            index = index + 1;

            if tracks_array.len() == 10 {
                break 'outer;
            }

            let builder = {
                if get_value() == 1 {
                    client
                        .post("http://localhost:5000/simulate")
                        .json(&vec![Req {
                            meta: Eveent {
                                event: "track".to_string(),
                                account: "municio".to_string(),
                            },
                            payload: Tracks {
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
                                received_at: Some(
                                    (Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                                ),
                                connection_id: tracks[index].get_i64("connection_id").unwrap()
                                    as i64,
                                index: tracks[index].get_i64("index").unwrap() as i64,
                                fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                                url: Some(tracks[index].get_str("url").unwrap().to_string()),
                            },
                        }])
                } else {
                    client
                        .post("http://localhost:5000/simulate")
                        .json(&tracks_array)
                }
            };
            let res = builder.send().await;

            match res {
                Ok(_a) => match get_value() {
                    1 => continue,
                    0 => {
                        tracks_array = vec![];
                        set_value(1);
                    }
                    _ => continue,
                },
                Err(_err) => match get_value() {
                    1 => {
                        tracks_array.push(Req {
                            meta: Eveent {
                                event: "track".to_string(),
                                account: "municio".to_string(),
                            },
                            payload: Tracks {
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
                                received_at: Some(
                                    (Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                                ),
                                connection_id: tracks[index].get_i64("connection_id").unwrap()
                                    as i64,
                                index: tracks[index].get_i64("index").unwrap() as i64,
                                fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                                url: Some(tracks[index].get_str("url").unwrap().to_string()),
                            },
                        });
                        set_value(0);
                    }
                    0 => tracks_array.push(Req {
                        meta: Eveent {
                            event: "track".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Tracks {
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
                            received_at: Some(
                                (Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                            ),
                            connection_id: tracks[index].get_i64("connection_id").unwrap() as i64,
                            index: tracks[index].get_i64("index").unwrap() as i64,
                            fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                            url: Some(tracks[index].get_str("url").unwrap().to_string()),
                        },
                    }),
                    _ => continue,
                },
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

#[get("/<msg>")]
pub async fn index(msg: String) -> Template {
    use mongodb::bson::doc;

    let client = get_client().await.unwrap();

    let pres_collection: Collection<Presence> = client.database("munic").collection("presences");
    let track_collection: Collection<Tracks> = client.database("munic").collection("tracks");

    let trk_pipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$sort": { "date" : 1 }},
        doc! {"$group":  {
            "_id": { "year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
        }},
    ];
    let pres_pipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$time" } }  },
        doc! {"$sort": { "date" : 1 }},
        doc! {"$group":  {
            "_id": { "year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
        }},
    ];

    let tracks_data = track_collection
        .aggregate(trk_pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let presence_data = pres_collection
        .aggregate(pres_pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use futures::stream::TryStreamExt;

    let track_dates: Vec<_> = tracks_data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();
    let presence_dates: Vec<_> = presence_data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    Template::render(
        "index",
        context! {msg:msg,presence_dates:presence_dates,track_dates:track_dates},
    )
}

#[get("/")]
pub async fn indexx() -> Template {
    use mongodb::bson::doc;

    let client = get_client().await.unwrap();

    let pres_collection: Collection<Presence> = client.database("munic").collection("presences");
    let track_collection: Collection<Tracks> = client.database("munic").collection("tracks");

    let trk_pipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$sort": { "date" : 1 }},
        doc! {"$group":  {
            "_id": { "file": "$file","year": { "$year": "$date" },  "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
        }},
    ];
    let pres_pipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$time" } }  },
        doc! {"$sort": { "date" : 1 }},
        doc! {"$group":  {
            "_id": { "year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
        }},
    ];

    let tracks_data = track_collection
        .aggregate(trk_pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let presence_data = pres_collection
        .aggregate(pres_pipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use futures::stream::TryStreamExt;

    let track_dates: Vec<_> = tracks_data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let presence_dates: Vec<_> = presence_data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    // println!("{:?}", track_dates);

    Template::render(
        "index",
        context! {msg:"",presence_dates:presence_dates,track_dates:track_dates},
    )
}

async fn get_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    use dotenvy::dotenv;
    dotenv().ok();
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

// #[get("/store_presence")]
// pub async fn store_p() -> Redirect {
//     tokio::spawn(async move {
//         match store_presence().await {
//             Ok(_e) => (),
//             Err(_e) => panic!("presence storage panic !!"),
//         }
//     });
//     Redirect::to(uri!(index("Presences Stored Successfully")))
// }

// #[get("/store_tracks")]
// pub async fn store_t() -> Redirect {
//     tokio::spawn(async move { store_tracks().await });
//     Redirect::to(uri!(index("Tracks Stored Successfully")))
// }

async fn store_presence(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    use dotenvy::dotenv;
    dotenv().ok();

    let data = match file {
        Some(file_name) => {
            fs::read_to_string("./uploads/".to_owned() + file_name).expect("Unable to read file")
        }
        None => "".to_owned(),
    };

    let json_data: Vec<Presence> = serde_json::from_str(&data).expect("Unable to read file");

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection = client.database("munic").collection("presences");

    for presence in json_data {
        match collection.insert_one(presence, None).await {
            Ok(_e) => continue,
            Err(_e) => panic!("presence storage panic !!"),
        }
    }
    Ok(())
}

async fn store_tracks(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    use dotenvy::dotenv;
    dotenv().ok();

    let data = match file {
        Some(file_name) => {
            fs::read_to_string("./uploads/".to_owned() + file_name).expect("Unable to read file")
        }
        None => "".to_owned(),
    };

    let json_data: Vec<Tracks> = serde_json::from_str(&data).expect("Unable to read file");

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection = client.database("munic").collection("tracks");

    for track in json_data {
        match collection.insert_one(track, None).await {
            Ok(_e) => continue,
            Err(_e) => panic!("track storage panic !!"),
        }
    }
    Ok(())
}

async fn update_tracks(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection: Collection<Tracks> = client.database("munic").collection("tracks");

    let filter = doc! {"file":{"$exists":false}};
    let update = doc! {"$set": {"file":file}};
    collection.update_many(filter, update, None).await.unwrap();
    Ok(())
}

async fn update_presence(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let collection: Collection<Tracks> = client.database("munic").collection("presences");

    let filter = doc! {"file":{"$exists":false}};
    let update = doc! {"$set": {"file":file}};
    collection.update_many(filter, update, None).await.unwrap();
    Ok(())
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn ping_server(url: String) -> bool {
    use dns_lookup::lookup_host;
    use winping::{Buffer, Pinger};

    let u: Url = Url::parse(&url).unwrap();

    let ips: Vec<std::net::IpAddr> = lookup_host(&u.host().unwrap().to_string()).unwrap();

    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();
    match pinger.send(ips[0], &mut buffer) {
        Ok(_) => true,
        Err(_err) => false,
    }
}

#[derive(FromForm, Debug)]
pub struct UserInput {
    lon: String,
    lat: String,
    url: String,
    track_option: String,
    presence_option: String,
}
