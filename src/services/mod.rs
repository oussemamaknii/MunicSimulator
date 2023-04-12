//service/mod.rs

extern crate chrono;
extern crate rocket;
use crate::models::{self, Presence, Tracks};
use atomic_array::AtomicOptionRefArray;
use bson::{doc, Bson, Document};
use chrono::DateTime;
use mongodb::Collection;
use once_cell::sync::Lazy;
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::uri;
use std::fs;
use tokio::task::JoinHandle;

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

use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use tokio::time::{self, Duration};
use url::Url;

use core::sync::atomic::{AtomicUsize, Ordering};

static THREADS: Lazy<AtomicOptionRefArray<(String, JoinHandle<()>, Sender<()>, Sender<()>)>> =
    Lazy::new(|| AtomicOptionRefArray::new(10));

static STATE: AtomicUsize = AtomicUsize::new(3);

static INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn set_value(val: usize) {
    STATE.store(val, Ordering::Relaxed)
}

pub fn get_value() -> usize {
    STATE.load(Ordering::Relaxed)
}

#[post("/upload", data = "<data>")]
pub async fn file_json(content_type: &ContentType, data: Data<'_>) -> Redirect {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("track_json_file")
            .content_type_by_string(Some(mime::APPLICATION_JSON))
            .unwrap(),
        MultipartFormDataField::file("presence_json_file")
            .content_type_by_string(Some(mime::APPLICATION_JSON))
            .unwrap(),
    ]);

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    let track_json_file = multipart_form_data.files.get("track_json_file");
    let presence_json_file = multipart_form_data.files.get("presence_json_file");

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
            None => (),
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
            None => (),
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
    Redirect::to(uri!(index("Files stored Successfully!")))
}

#[post("/simulate", data = "<user_input>")]
pub async fn simulate(content_type: &ContentType, user_input: Data<'_>) -> Redirect {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("url"),
        MultipartFormDataField::text("lon"),
        MultipartFormDataField::text("lat"),
        MultipartFormDataField::text("key"),
        MultipartFormDataField::text("track_option"),
        MultipartFormDataField::text("presence_option"),
        MultipartFormDataField::text("track_file"),
        MultipartFormDataField::text("presence_file"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, user_input, options)
        .await
        .unwrap();

    let url_text = multipart_form_data.texts.remove("url");
    let mut turl_text: String = "".to_string();
    let mut turl_thread: String = "".to_string();
    let lon_text = multipart_form_data.texts.remove("lon");
    let mut tlon_text: String = "".to_string();
    let lat_text = multipart_form_data.texts.remove("lat");
    let mut tlat_text: String = "".to_string();
    let track_option = multipart_form_data.texts.remove("track_option");
    let mut ttrack_option: String = "".to_string();
    let presence_option = multipart_form_data.texts.remove("presence_option");
    let mut tpresence_option: String = "".to_string();
    let track_file = multipart_form_data.texts.remove("track_file");
    let mut ttrack_file: String = "".to_string();
    let presence_file = multipart_form_data.texts.remove("presence_file");
    let mut tpresence_file: String = "".to_string();
    let key = multipart_form_data.texts.remove("key");
    let mut tkey: String = "".to_string();

    if let Some(mut url_text) = url_text {
        let text_field = url_text.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        turl_thread = _text.clone();
        turl_text = String::from(_text);
    }
    if let Some(mut lon_text) = lon_text {
        let text_field = lon_text.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tlon_text = String::from(_text);
    }
    if let Some(mut lat_text) = lat_text {
        let text_field = lat_text.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tlat_text = String::from(_text);
    }
    if let Some(mut track_option) = track_option {
        let text_field = track_option.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        ttrack_option = String::from(_text);
    }
    if let Some(mut presence_option) = presence_option {
        let text_field = presence_option.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tpresence_option = String::from(_text);
    }
    if let Some(mut track_file) = track_file {
        let text_field = track_file.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        ttrack_file = String::from(_text);
    }
    if let Some(mut presence_file) = presence_file {
        let text_field = presence_file.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tpresence_file = String::from(_text);
    }
    if let Some(mut key) = key {
        let text_field = key.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tkey = String::from(_text);
    }

    if ttrack_option.len() != 10 {
        if ttrack_option.match_indices("-").nth(1) == Some((6, "-")) {
            ttrack_option.insert(5, '0');
        }
        if ttrack_option.match_indices("-").nth(1) == Some((7, "-")) && ttrack_option.len() != 10 {
            ttrack_option.insert(8, '0');
        }
    }

    if tpresence_option.len() != 10 {
        if tpresence_option.match_indices("-").nth(1) == Some((6, "-")) {
            tpresence_option.insert(5, '0');
        }
        if tpresence_option.match_indices("-").nth(1) == Some((7, "-"))
            && tpresence_option.len() != 10
        {
            tpresence_option.insert(8, '0');
        }
    }

    if ping_server(turl_text.clone()) {
        set_value(1);
        tokio::spawn(async move {
            // use futures::future::join_all;
            // let mut handles = vec![];

            let (tsender, treceiver) = channel();
            let (psender, preceiver) = channel();

            let worker = tokio::spawn(async move {
                use google_maps::prelude::*;
                if tkey != "" {
                    let google_maps_client = GoogleMapsClient::new(&tkey);
                    let directions = google_maps_client
                        .directions(
                            Location::Address(String::from(tlat_text)),
                            Location::Address(String::from(tlon_text)),
                            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
                        )
                        .with_travel_mode(TravelMode::Driving)
                        .execute()
                        .await;

                    send_tracks(directions, &turl_text, &ttrack_option).await;
                } else {
                    replay(
                        &turl_text.clone(),
                        &ttrack_option,
                        &tpresence_option,
                        &ttrack_file,
                        &tpresence_file,
                        treceiver,
                        preceiver,
                    )
                    .await;
                }

                println!("Work Done !")
            });

            THREADS.store(
                INDEX.load(Ordering::Relaxed),
                (turl_thread, worker, tsender, psender),
            );
            INDEX.store(INDEX.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
            // handles.push(worker);
            // join_all(handles).await;
            println!("Shuting down the Request Handler thread!!")
        });

        return Redirect::to(uri!(index("Simulating !")));
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

    let pipeline = vec![
        doc! {"$match": { "fields": { "$gt": {} } }  },
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$addFields": { "subs": {"$substr": [ "$recorded_at", 0, 10 ]} }},
        doc! {
            "$match": { "subs" :  {"$eq": option} }
        },
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

            index = index + 1;

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

async fn replay(
    url: &String,
    track_option: &String,
    presence_option: &String,
    track_file: &String,
    presence_file: &String,
    treceiver: Receiver<()>,
    preceiver: Receiver<()>,
) -> () {
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

    let tpipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
        doc! {"$addFields": { "subs": {"$substr": [ "$recorded_at", 0, 10 ]} }},
        doc! {
            "$match": { "subs" :  {"$eq": track_option} }
        },
        doc! {
            "$match": { "file" :  {"$eq": track_file} }
        },
        doc! {"$sort": { "date" : 1 }},
    ];

    let presence_collection = client.database("munic").collection::<Presence>("presences");

    let ppipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$time" } }  },
        doc! {"$addFields": { "subs": {"$substr": [ "$time", 0, 10 ]} }},
        doc! {
            "$match": { "subs" :  {"$eq": presence_option} }
        },
        doc! {
            "$match": { "file" :  {"$eq": presence_file} }
        },
        doc! {"$sort": { "date" : 1 }},
    ];

    let tracks_data = tracks_collection
        .aggregate(tpipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();
    let presence_data = presence_collection
        .aggregate(ppipeline, None)
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    use futures::stream::TryStreamExt;

    let tracks: Vec<_> = tracks_data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();
    let presences: Vec<_> = presence_data
        .try_collect()
        .await
        .map_err(|e| println!("{}", e))
        .unwrap();

    let track_client = reqwest::Client::new();
    let presence_client = reqwest::Client::new();

    use futures::future::join_all;
    let mut handles = vec![];
    let url_clone = url.clone();
    let url_clonee = url.clone();

    let presence_worker = tokio::spawn(async move {
        replay_presence(presences, presence_client, &url_clonee, preceiver).await;
    });

    let track_worker = tokio::spawn(async move {
        replay_tracks(tracks, track_client, &url_clone, treceiver).await;
    });

    handles.push(presence_worker);

    handles.push(track_worker);

    join_all(handles).await;

    println!("Shuting down replay the Handler thread!!")
}

pub async fn replay_tracks(
    tracks: Vec<Document>,
    track_client: reqwest::Client,
    url: &String,
    receiver: Receiver<()>,
) {
    let mut tracks_array: Vec<Req<Tracks>> = vec![];

    let a = vec![Bson::Double(0.0), Bson::Double(0.0)];
    static DEF: Option<[f64; 2]> = None;

    let mut old_track: Document = Document::new();

    let mut first = true;

    'outer: for track in tracks {
        match receiver.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break 'outer;
            }
            Err(TryRecvError::Empty) => {}
        }

        if first == true {
            time::sleep(Duration::from_secs(1)).await;
            first = false;
        } else {
            let t1 =
                DateTime::parse_from_rfc3339(&track.get_str("recorded_at").unwrap().to_string())
                    .unwrap();
            let t2 = DateTime::parse_from_rfc3339(
                &old_track.get_str("recorded_at").unwrap().to_string(),
            )
            .unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            time::sleep(Duration::from_secs(elapsed_seconds as u64)).await;
        }

        old_track = track.clone();

        let location = track.get_array("location").unwrap_or_else(|_| &a);
        let loc = track.get_array("loc").unwrap_or_else(|_| &a);

        println!("sending tracks...");

        if tracks_array.len() == 10 {
            break 'outer;
        }

        let builder = {
            if get_value() == 1 {
                track_client.post(url).json(&vec![Req {
                    meta: Eveent {
                        event: "track".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Tracks {
                        id: track.get_i64("id").unwrap() as i64,
                        id_str: Some(track.get_str("id_str").unwrap().to_string()),
                        location: match [
                            location[0].as_f64().unwrap(),
                            location[1].as_f64().unwrap(),
                        ] {
                            e => {
                                if e != [0.0, 0.0] {
                                    Some(e)
                                } else {
                                    DEF
                                }
                            }
                        },
                        loc: match [loc[0].as_f64().unwrap(), loc[1].as_f64().unwrap()] {
                            e => {
                                if e != [0.0, 0.0] {
                                    Some(e)
                                } else {
                                    DEF
                                }
                            }
                        },
                        asset: Some(track.get_str("asset").unwrap().to_string()),
                        recorded_at: Some(track.get_str("recorded_at").unwrap().to_string()),
                        recorded_at_ms: Some(track.get_str("recorded_at_ms").unwrap().to_string()),
                        received_at: Some(track.get_str("received_at").unwrap().to_string()),
                        connection_id: track.get_i64("connection_id").unwrap() as i64,
                        index: track.get_i64("index").unwrap() as i64,
                        fields: Fields::from(track.get_document("fields").unwrap()),
                        url: Some(track.get_str("url").unwrap().to_string()),
                    },
                }])
            } else {
                track_client.post(url).json(&tracks_array)
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
                            id: track.get_i64("id").unwrap() as i64,
                            id_str: Some(track.get_str("id_str").unwrap().to_string()),
                            location: match [
                                location[0].as_f64().unwrap(),
                                location[1].as_f64().unwrap(),
                            ] {
                                e => {
                                    if e != [0.0, 0.0] {
                                        Some(e)
                                    } else {
                                        DEF
                                    }
                                }
                            },
                            loc: match [loc[0].as_f64().unwrap(), loc[1].as_f64().unwrap()] {
                                e => {
                                    if e != [0.0, 0.0] {
                                        Some(e)
                                    } else {
                                        DEF
                                    }
                                }
                            },
                            asset: Some(track.get_str("asset").unwrap().to_string()),
                            recorded_at: Some(track.get_str("recorded_at").unwrap().to_string()),
                            recorded_at_ms: Some(
                                track.get_str("recorded_at_ms").unwrap().to_string(),
                            ),
                            received_at: Some(track.get_str("received_at").unwrap().to_string()),
                            connection_id: track.get_i64("connection_id").unwrap() as i64,
                            index: track.get_i64("index").unwrap() as i64,
                            fields: Fields::from(track.get_document("fields").unwrap()),
                            url: Some(track.get_str("url").unwrap().to_string()),
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
                        id: track.get_i64("id").unwrap() as i64,
                        id_str: Some(track.get_str("id_str").unwrap().to_string()),
                        location: Some([
                            location[0].as_f64().unwrap(),
                            location[1].as_f64().unwrap(),
                        ]),
                        loc: Some([loc[0].as_f64().unwrap(), loc[1].as_f64().unwrap()]),
                        asset: Some(track.get_str("asset").unwrap().to_string()),
                        recorded_at: Some(track.get_str("recorded_at").unwrap().to_string()),
                        recorded_at_ms: Some(track.get_str("recorded_at_ms").unwrap().to_string()),
                        received_at: Some(track.get_str("received_at").unwrap().to_string()),
                        connection_id: track.get_i64("connection_id").unwrap() as i64,
                        index: track.get_i64("index").unwrap() as i64,
                        fields: Fields::from(track.get_document("fields").unwrap()),
                        url: Some(track.get_str("url").unwrap().to_string()),
                    },
                }),
                _ => continue,
            },
        }
    }
    println!("Shuting down tracks replay the Handler thread!!")
}

pub async fn replay_presence(
    presences: Vec<Document>,
    presence_client: reqwest::Client,
    url: &String,
    receiver: Receiver<()>,
) {
    let mut presences_array: Vec<Req<Presence>> = vec![];

    let mut old_pres: Document = Document::new();

    let mut first = true;

    'outer: for presence in presences {
        match receiver.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break 'outer;
            }
            Err(TryRecvError::Empty) => {}
        }
        if first == true {
            time::sleep(Duration::from_secs(1)).await;
            first = false;
        } else {
            let t1 = DateTime::parse_from_rfc3339(&presence.get_str("time").unwrap().to_string())
                .unwrap();
            let t2 = DateTime::parse_from_rfc3339(&old_pres.get_str("time").unwrap().to_string())
                .unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            time::sleep(Duration::from_secs(elapsed_seconds as u64)).await;
        }

        old_pres = presence.clone();

        println!("sending presence...");

        if presences_array.len() == 10 {
            break 'outer;
        }

        let builder = {
            if get_value() == 1 {
                presence_client.post(url).json(&vec![Req {
                    meta: Eveent {
                        event: "presence".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Presence {
                        id: presence.get_i64("id").unwrap() as i64,
                        connection_id: presence.get_i64("connection_id").unwrap() as i64,
                        id_str: match presence.get_str("id_str") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        typ: match presence.get_str("type") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        fullreason: match presence.get_str("fullreason") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        cs: match presence.get_str("cs") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        ip: match presence.get_str("ip") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        protocol: match presence.get_str("protocol") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        reason: match presence.get_str("reason") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        asset: match presence.get_str("asset") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        time: match presence.get_str("time") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                    },
                }])
            } else {
                presence_client.post(url).json(&presences_array)
            }
        };
        let res = builder.send().await;

        match res {
            Ok(_a) => match get_value() {
                1 => continue,
                0 => {
                    presences_array = vec![];
                    set_value(1);
                }
                _ => continue,
            },
            Err(_err) => match get_value() {
                1 => {
                    presences_array.push(Req {
                        meta: Eveent {
                            event: "presence".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Presence {
                            id: presence.get_i64("id").unwrap() as i64,
                            connection_id: presence.get_i64("connection_id").unwrap() as i64,
                            id_str: match presence.get_str("id_str") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            typ: match presence.get_str("type") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            fullreason: match presence.get_str("fullreason") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            cs: match presence.get_str("cs") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            ip: match presence.get_str("ip") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            protocol: match presence.get_str("protocol") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            reason: match presence.get_str("reason") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            asset: match presence.get_str("asset") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                            time: match presence.get_str("time") {
                                Ok(e) => Some(e.to_string()),
                                Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                            },
                        },
                    });
                    set_value(0);
                }
                0 => presences_array.push(Req {
                    meta: Eveent {
                        event: "presence".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Presence {
                        id: presence.get_i64("id").unwrap() as i64,
                        connection_id: presence.get_i64("connection_id").unwrap() as i64,
                        id_str: match presence.get_str("id_str") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        typ: match presence.get_str("type") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        fullreason: match presence.get_str("fullreason") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        cs: match presence.get_str("cs") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        ip: match presence.get_str("ip") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        protocol: match presence.get_str("protocol") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        reason: match presence.get_str("reason") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        asset: match presence.get_str("asset") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                        time: match presence.get_str("time") {
                            Ok(e) => Some(e.to_string()),
                            Err(_) => Some(String::new()).filter(|s| !s.is_empty()),
                        },
                    },
                }),
                _ => continue,
            },
        }
    }
    println!("Shuting down presence replay the Handler thread!!")
}

// #[get("/test")]
// pub async fn test() -> () {
//     use dotenvy::dotenv;
//     dotenv().ok();

//     let client_uri =
//         env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

//     let options =
//         ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
//             .await
//             .unwrap();
//     let client = mongodb::Client::with_options(options).unwrap();

//     let tracks_collection = client.database("munic").collection::<Tracks>("tracks");

//     let pipeline = vec![
//         doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
//         doc! {"$addFields": { "subs": {"$substr": [ "$recorded_at", 0, 10 ]} }},
//         doc! {
//             "$match": { "subs" :  {"$eq": "2023-01-25"} }
//         },
//         doc! {"$sort": { "date" : 1 }},
//     ];

//     let data = tracks_collection
//         .aggregate(pipeline, None)
//         .await
//         .map_err(|e| println!("{}", e))
//         .unwrap();

//     use futures::stream::TryStreamExt;

//     let tracks: Vec<_> = data
//         .try_collect()
//         .await
//         .map_err(|e| println!("{}", e))
//         .unwrap();

//     let client = reqwest::Client::new();

//     let mut tracks_array: Vec<Req<Tracks>> = vec![];

//     set_value(1);

//     let mut ten_minutes = time::interval(Duration::from_secs_f64(5.1).try_into().unwrap());

//     let a = vec![Bson::Double(0.0), Bson::Double(0.0)];
//     static DEF: Option<[f64; 2]> = None;

//     'outer: for track in tracks {
//         let location = track.get_array("location").unwrap_or_else(|_| &a);
//         let loc = track.get_array("loc").unwrap_or_else(|_| &a);

//         ten_minutes.tick().await;
//         println!("sending ...");

//         if tracks_array.len() == 10 {
//             break 'outer;
//         }

//         let builder = {
//             if get_value() == 1 {
//                 client.post("http://localhost:5000/").json(&vec![Req {
//                     meta: Eveent {
//                         event: "track".to_string(),
//                         account: "municio".to_string(),
//                     },
//                     payload: Tracks {
//                         id: track.get_i64("id").unwrap() as i64,
//                         id_str: Some(track.get_str("id_str").unwrap().to_string()),
//                         location: match [
//                             location[0].as_f64().unwrap(),
//                             location[1].as_f64().unwrap(),
//                         ] {
//                             e => {
//                                 if e != [0.0, 0.0] {
//                                     Some(e)
//                                 } else {
//                                     DEF
//                                 }
//                             }
//                         },
//                         loc: match [loc[0].as_f64().unwrap(), loc[1].as_f64().unwrap()] {
//                             e => {
//                                 if e != [0.0, 0.0] {
//                                     Some(e)
//                                 } else {
//                                     DEF
//                                 }
//                             }
//                         },
//                         asset: Some(track.get_str("asset").unwrap().to_string()),
//                         recorded_at: Some(track.get_str("recorded_at").unwrap().to_string()),
//                         recorded_at_ms: Some(track.get_str("recorded_at_ms").unwrap().to_string()),
//                         received_at: Some(track.get_str("received_at").unwrap().to_string()),
//                         connection_id: track.get_i64("connection_id").unwrap() as i64,
//                         index: track.get_i64("index").unwrap() as i64,
//                         fields: Fields::from(track.get_document("fields").unwrap()),
//                         url: Some(track.get_str("url").unwrap().to_string()),
//                     },
//                 }])
//             } else {
//                 client.post("http://localhost:5000/").json(&tracks_array)
//             }
//         };
//         let res = builder.send().await;

//         match res {
//             Ok(_a) => match get_value() {
//                 1 => continue,
//                 0 => {
//                     tracks_array = vec![];
//                     set_value(1);
//                 }
//                 _ => continue,
//             },
//             Err(_err) => match get_value() {
//                 1 => {
//                     tracks_array.push(Req {
//                         meta: Eveent {
//                             event: "track".to_string(),
//                             account: "municio".to_string(),
//                         },
//                         payload: Tracks {
//                             id: track.get_i64("id").unwrap() as i64,
//                             id_str: Some(track.get_str("id_str").unwrap().to_string()),
//                             location: match [
//                                 location[0].as_f64().unwrap(),
//                                 location[1].as_f64().unwrap(),
//                             ] {
//                                 e => {
//                                     if e != [0.0, 0.0] {
//                                         Some(e)
//                                     } else {
//                                         DEF
//                                     }
//                                 }
//                             },
//                             loc: match [loc[0].as_f64().unwrap(), loc[1].as_f64().unwrap()] {
//                                 e => {
//                                     if e != [0.0, 0.0] {
//                                         Some(e)
//                                     } else {
//                                         DEF
//                                     }
//                                 }
//                             },
//                             asset: Some(track.get_str("asset").unwrap().to_string()),
//                             recorded_at: Some(track.get_str("recorded_at").unwrap().to_string()),
//                             recorded_at_ms: Some(
//                                 track.get_str("recorded_at_ms").unwrap().to_string(),
//                             ),
//                             received_at: Some(track.get_str("received_at").unwrap().to_string()),
//                             connection_id: track.get_i64("connection_id").unwrap() as i64,
//                             index: track.get_i64("index").unwrap() as i64,
//                             fields: Fields::from(track.get_document("fields").unwrap()),
//                             url: Some(track.get_str("url").unwrap().to_string()),
//                         },
//                     });
//                     set_value(0);
//                 }
//                 0 => tracks_array.push(Req {
//                     meta: Eveent {
//                         event: "track".to_string(),
//                         account: "municio".to_string(),
//                     },
//                     payload: Tracks {
//                         id: track.get_i64("id").unwrap() as i64,
//                         id_str: Some(track.get_str("id_str").unwrap().to_string()),
//                         location: Some([
//                             location[0].as_f64().unwrap(),
//                             location[1].as_f64().unwrap(),
//                         ]),
//                         loc: Some([loc[0].as_f64().unwrap(), loc[1].as_f64().unwrap()]),
//                         asset: Some(track.get_str("asset").unwrap().to_string()),
//                         recorded_at: Some(track.get_str("recorded_at").unwrap().to_string()),
//                         recorded_at_ms: Some(track.get_str("recorded_at_ms").unwrap().to_string()),
//                         received_at: Some(track.get_str("received_at").unwrap().to_string()),
//                         connection_id: track.get_i64("connection_id").unwrap() as i64,
//                         index: track.get_i64("index").unwrap() as i64,
//                         fields: Fields::from(track.get_document("fields").unwrap()),
//                         url: Some(track.get_str("url").unwrap().to_string()),
//                     },
//                 }),
//                 _ => continue,
//             },
//         }
//     }
// }

#[get("/test")]
pub async fn test() -> () {
    use google_maps::prelude::*;
    use polyline;

    let google_maps_client = GoogleMapsClient::new("AIzaSyAo1agGjrUSZhLwPydiX-_dJ-CEQkxoRmU");
    let directions = google_maps_client
        .directions(
            Location::Address(String::from("ariana tunisie")),
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

            if tracks_array.len() == 10 {
                break 'outer;
            }

            let builder = {
                if get_value() == 1 {
                    client.post("http://localhost:5000/").json(&vec![Req {
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
                    client.post("http://localhost:5000/").json(&tracks_array)
                }
            };
            let res = builder.send().await;

            index = index + 1;

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

#[post("/abort", data = "<url>")]
pub fn abort(url: String) {
    for index in 0..THREADS.len() {
        match THREADS.load(index) {
            Some(e) => {
                if e.0 == url {
                    let _ = &e.1.abort();
                    match e.2.send(()) {
                        Ok(_e) => println!("Terminating tracks signal !"),
                        Err(e) => println!("{:?}", e),
                    }
                    match e.3.send(()) {
                        Ok(_e) => println!("Terminating presences signal !"),
                        Err(e) => println!("{:?}", e),
                    }
                    THREADS.store(index, None);
                }
            }
            None => (),
        }
    }
}

#[get("/events")]
pub fn stream() -> EventStream![] {
    use serde_json::{Map, Value};
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(2));

        loop {
            let mut arr = Vec::<String>::new();
            for index in 0..THREADS.len(){
                match THREADS.load(index){
                    Some(e)=> arr.push(e.0.clone()),
                    None=>()
                }
            }
            let mut map = Map::new();
            map.insert("threads".to_string(), arr.into());
            map.insert("http".to_string(),Value::String( get_value().to_string()));
            yield Event::json(&map);
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
            "_id": { "file": "$file","year": { "$year": "$date" },  "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
        }},
    ];
    let pres_pipeline = vec![
        doc! {"$addFields": { "date": { "$toDate": "$time" } }  },
        doc! {"$sort": { "date" : 1 }},
        doc! {"$group":  {
            "_id": { "file": "$file","year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
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
            "_id": { "file": "$file","year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
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
