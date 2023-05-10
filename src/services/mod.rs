//service/mod.rs

extern crate chrono;
extern crate rocket;
use crate::models::{self, Base64, Presence, Tracks};
use atomic_array::AtomicOptionRefArray;
use bson::{doc, Bson, Document};
use chrono::{DateTime, Local};
use mongodb::Collection;
use once_cell::sync::Lazy;
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::uri;
use std::fs::{self, OpenOptions};
use std::sync::atomic::AtomicI8;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::task::JoinHandle;
type GDirection = core::result::Result<
    google_maps::directions::response::Response,
    google_maps::directions::error::Error,
>;
use core::sync::atomic::{AtomicUsize, Ordering};
use dotenvy::dotenv;
use models::{Eveent, Fields, Req};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Data;
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::{env, mem, thread};
use tokio::time::{self, Duration};
use url::Url;

static THREADS: Lazy<
    AtomicOptionRefArray<(
        String,
        JoinHandle<()>,
        Sender<()>,
        Sender<()>,
        AtomicI8,
        Sender<()>,
    )>,
> = Lazy::new(|| AtomicOptionRefArray::new(10));

static INDEX: AtomicUsize = AtomicUsize::new(0);

static START: AtomicUsize = AtomicUsize::new(0);

static RECORD: AtomicUsize = AtomicUsize::new(0);

#[post("/record")]
pub fn record() {
    if RECORD.load(Ordering::Relaxed) == 0 {
        RECORD.store(1, Ordering::Relaxed);
    } else {
        RECORD.store(0, Ordering::Relaxed);
    }
}
#[post("/", format = "json", data = "<json_data>")]
pub async fn notif(json_data: Json<serde_json::Value>) -> rocket::response::status::Custom<String> {
    if RECORD.load(Ordering::Relaxed) == 1 {
        use std::fs::{self};
        use std::io::prelude::*;
        use std::path::{Path, PathBuf};
        dotenv().ok();

        let paaath = format!("{}{}", env::var("DIR").unwrap(), "uploads\\");
        let json_value = json_data.into_inner();

        let dir_path = Path::new(&paaath);
        let prefix = "trip_";
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let file_extension = ".json";

        // Count the number of files in the directory with the "trip_" prefix
        let file_count = fs::read_dir(dir_path)
            .expect("Failed to read directory")
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.file_name().to_string_lossy().starts_with(prefix)
                    && entry.file_name().to_string_lossy()[entry.file_name().to_string_lossy().len()
                        - 15
                        ..entry.file_name().to_string_lossy().len() - 5]
                        == date
            })
            .count();

        // Construct the file name with the format "trip_%i_%date.json"
        let file_name: String;
        if file_count == 0 {
            file_name = format!("{}{}_{}{}", prefix, file_count + 1, date, file_extension);
        } else {
            file_name = format!("{}{}_{}{}", prefix, file_count, date, file_extension);
        }
        let file_path = PathBuf::from(dir_path).join(file_name);

        // Check if the file already exists

        if file_path.exists() {
            // file exists
            // Check if the file is empty
            let is_empty = file_path.metadata().map(|m| m.len() == 0).unwrap_or(false);

            if is_empty {
                // file empty
                let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&file_path)
                    .expect("Failed to open file");

                let file_mutex = Arc::new(Mutex::new(file));
                let file_mutex_clone = Arc::clone(&file_mutex);
                let mut file = file_mutex_clone.lock().unwrap();

                match json_value {
                    serde_json::Value::Object(_obj) => {
                        // Handle a JSON object.
                        return rocket::response::status::Custom(
                            Status::Ok,
                            "Processed JSON data successfully".to_string(),
                        );
                    }
                    serde_json::Value::Array(arr) => {
                        // Handle a JSON array.

                        for json_obj in Json(arr).into_inner() {
                            // Open the file in append mode
                            match json_obj["payload"].get("type") {
                                Some(e) => {
                                    if e.as_str().unwrap() == "connect" {
                                        file.write_all(b"[").expect("failed to write to file");
                                        START.store(1, Ordering::Relaxed);
                                    } else if e.as_str().unwrap() == "disconnect" {
                                        let mut byte_array = serde_json::to_vec(&json_obj).unwrap();
                                        byte_array.push(b']');

                                        file.write(&byte_array[..])
                                            .expect("failed to write to file");
                                        START.store(0, Ordering::Relaxed);
                                    }
                                }
                                None => (),
                            }

                            if START.load(Ordering::Relaxed) == 1 {
                                let mut byte_array = serde_json::to_vec(&json_obj).unwrap();

                                byte_array.push(b',');

                                file.write_all(&byte_array[..])
                                    .expect("failed to write to file");
                            }
                        }
                        return rocket::response::status::Custom(
                            Status::Ok,
                            "Processed JSON data successfully".to_string(),
                        );
                    }
                    _ => {
                        // Handle any other type of JSON value.
                        return rocket::response::status::Custom(
                            Status::BadRequest,
                            "Bad Request".to_string(),
                        );
                    }
                }
            } else {
                // file not empty
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .read(true)
                    .open(&file_path)
                    .expect("Failed to open file");

                let mut contents = String::new();

                file.read_to_string(&mut contents)
                    .expect("Failed to read file");

                let file_mutex = Arc::new(Mutex::new(file));
                let file_mutex_clone = Arc::clone(&file_mutex);
                let mut file = file_mutex_clone.lock().unwrap();

                if contents.ends_with(']') {
                    // not empty and Check if the last character of the file is a ']'
                    // Increment the file count and create a new file with the incremented %i
                    let new_file_count = file_count + 1;
                    let new_file_name =
                        format!("{}{}_{}{}", prefix, new_file_count, date, file_extension);
                    let new_file_path = PathBuf::from(dir_path).join(new_file_name);

                    let new_file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open(&new_file_path)
                        .expect("Failed to create file");

                    let file_mutex = Arc::new(Mutex::new(new_file));
                    let file_mutex_clone = Arc::clone(&file_mutex);
                    let mut new_file = file_mutex_clone.lock().unwrap();

                    match json_value {
                        serde_json::Value::Object(_obj) => {
                            // Handle a JSON object.
                            return rocket::response::status::Custom(
                                Status::Ok,
                                "Processed JSON data successfully".to_string(),
                            );
                        }
                        serde_json::Value::Array(arr) => {
                            // Handle a JSON array.

                            for json_obj in Json(arr).into_inner() {
                                // Open the file in append mode
                                match json_obj["payload"].get("type") {
                                    Some(e) => {
                                        if e.as_str().unwrap() == "connect" {
                                            new_file
                                                .write_all(b"[")
                                                .expect("failed to write to file");
                                            START.store(1, Ordering::Relaxed);
                                        } else if e.as_str().unwrap() == "disconnect" {
                                            let mut byte_array =
                                                serde_json::to_vec(&json_obj).unwrap();
                                            byte_array.push(b']');

                                            new_file
                                                .write(&byte_array[..])
                                                .expect("failed to write to file");
                                            START.store(0, Ordering::Relaxed);
                                        }
                                    }
                                    None => (),
                                }

                                if START.load(Ordering::Relaxed) == 1 {
                                    let mut byte_array = serde_json::to_vec(&json_obj).unwrap();

                                    byte_array.push(b',');

                                    new_file
                                        .write(&byte_array[..])
                                        .expect("failed to write to file");
                                }
                            }
                            return rocket::response::status::Custom(
                                Status::Ok,
                                "Processed JSON data successfully".to_string(),
                            );
                        }
                        _ => {
                            // Handle any other type of JSON value.
                            return rocket::response::status::Custom(
                                Status::BadRequest,
                                "Bad Request".to_string(),
                            );
                        }
                    }
                } else {
                    // not empty and no ]
                    match json_value {
                        serde_json::Value::Object(_obj) => {
                            // Handle a JSON object.
                            return rocket::response::status::Custom(
                                Status::Ok,
                                "Processed JSON data successfully".to_string(),
                            );
                        }
                        serde_json::Value::Array(arr) => {
                            // Handle a JSON array.

                            for json_obj in Json(arr).into_inner() {
                                // Open the file in append mode
                                match json_obj["payload"].get("type") {
                                    Some(e) => {
                                        if e.as_str().unwrap() == "connect" {
                                            START.store(1, Ordering::Relaxed);
                                        } else if e.as_str().unwrap() == "disconnect" {
                                            let mut byte_array =
                                                serde_json::to_vec(&json_obj).unwrap();
                                            byte_array.push(b']');

                                            file.write(&byte_array[..])
                                                .expect("failed to write to file");
                                            START.store(0, Ordering::Relaxed);
                                        }
                                    }
                                    None => (),
                                }

                                if START.load(Ordering::Relaxed) == 1 {
                                    let mut byte_array = serde_json::to_vec(&json_obj).unwrap();

                                    byte_array.push(b',');

                                    file.write_all(&byte_array[..])
                                        .expect("failed to write to file");
                                }
                            }
                            return rocket::response::status::Custom(
                                Status::Ok,
                                "Processed JSON data successfully".to_string(),
                            );
                        }
                        _ => {
                            // Handle any other type of JSON value.
                            return rocket::response::status::Custom(
                                Status::BadRequest,
                                "Bad Request".to_string(),
                            );
                        }
                    }
                }
            }
        } else {
            // Create the file if it does not already exist
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&file_path)
                .expect("Failed to create file");

            let file_mutex = Arc::new(Mutex::new(file));
            let file_mutex_clone = Arc::clone(&file_mutex);
            let mut file = file_mutex_clone.lock().unwrap();

            match json_value {
                serde_json::Value::Object(_obj) => {
                    // Handle a JSON object.
                    return rocket::response::status::Custom(
                        Status::Ok,
                        "Processed JSON data successfully".to_string(),
                    );
                }
                serde_json::Value::Array(arr) => {
                    // Handle a JSON array.

                    for json_obj in Json(arr).into_inner() {
                        // Open the file in append mode
                        match json_obj["payload"].get("type") {
                            Some(e) => {
                                if e.as_str().unwrap() == "connect" {
                                    file.write_all(b"[").expect("failed to write to file");
                                    START.store(1, Ordering::Relaxed);
                                } else if e.as_str().unwrap() == "disconnect" {
                                    let mut byte_array = serde_json::to_vec(&json_obj).unwrap();
                                    byte_array.push(b']');

                                    file.write(&byte_array[..])
                                        .expect("failed to write to file");
                                    START.store(0, Ordering::Relaxed);
                                }
                            }
                            None => (),
                        }

                        if START.load(Ordering::Relaxed) == 1 {
                            let mut byte_array = serde_json::to_vec(&json_obj).unwrap();

                            byte_array.push(b',');

                            file.write_all(&byte_array[..])
                                .expect("failed to write to file");
                        }
                    }
                    return rocket::response::status::Custom(
                        Status::Ok,
                        "Processed JSON data successfully".to_string(),
                    );
                }
                _ => {
                    // Handle any other type of JSON value.
                    return rocket::response::status::Custom(
                        Status::BadRequest,
                        "Bad Request".to_string(),
                    );
                }
            }
        }
    }
    rocket::response::status::Custom(Status::Ok, "Record Disabled".to_string())
}

#[post("/upload", data = "<data>")]
pub async fn upload(content_type: &ContentType, data: Data<'_>) -> Redirect {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("json_file")
            .content_type_by_string(Some(mime::APPLICATION_JSON))
            .unwrap(),
    ]);

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    let json_file = multipart_form_data.files.get("json_file");

    if let Some(tfile_fields) = json_file {
        let file_field = &tfile_fields[0];

        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;

        dotenv().ok();
        let mut path = PathBuf::from(format!("{}{}", env::var("DIR").unwrap(), "uploads\\"));
        match _file_name {
            Some(name) => path.push(name),
            None => (),
        }

        match fs::rename(_path, &path) {
            Ok(_c) => (),
            Err(_e) => {
                let stem = path.file_stem().unwrap();
                let new_stem = format!("{}(1)", stem.to_string_lossy());
                path.set_file_name(new_stem);
                path.set_extension("json");
                fs::rename(_path, path).unwrap()
            }
        }
    }
    Redirect::to(uri!(index("File stored Successfully!")))
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
        MultipartFormDataField::text("chosen_json_file"),
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
    let chosen_json_file = multipart_form_data.texts.remove("chosen_json_file");
    let mut tchosen_json_file: String = "".to_string();
    let key = multipart_form_data.texts.remove("key");
    let mut tkey: String = "".to_string();

    if let Some(mut url_text) = url_text {
        let text_field = url_text.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        turl_thread = _text.clone();
        turl_text = _text;
    }
    if let Some(mut lon_text) = lon_text {
        let text_field = lon_text.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tlon_text = _text;
    }
    if let Some(mut lat_text) = lat_text {
        let text_field = lat_text.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tlat_text = _text;
    }
    if let Some(mut track_option) = track_option {
        let text_field = track_option.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        ttrack_option = _text;
    }
    if let Some(mut presence_option) = presence_option {
        let text_field = presence_option.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tpresence_option = _text;
    }
    if let Some(mut track_file) = track_file {
        let text_field = track_file.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        ttrack_file = _text;
    }
    if let Some(mut presence_file) = presence_file {
        let text_field = presence_file.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tpresence_file = _text;
    }
    if let Some(mut key) = key {
        let text_field = key.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tkey = _text;
    }
    if let Some(mut chosen_json_file) = chosen_json_file {
        let text_field = chosen_json_file.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        tchosen_json_file = _text;
    }

    let mut exists: bool = false;

    for index in 0..THREADS.len() {
        match THREADS.load(index) {
            Some(e) => {
                if e.0 == turl_text {
                    exists = true;
                    break;
                } else {
                    exists = false;
                }
            }
            None => exists = false,
        };
    }

    if ping_server(turl_text.clone()) && !exists {
        tokio::spawn(async move {
            if ttrack_option.len() != 10 {
                if ttrack_option.match_indices('-').nth(1) == Some((6, "-")) {
                    ttrack_option.insert(5, '0');
                }
                if ttrack_option.match_indices('-').nth(1) == Some((7, "-"))
                    && ttrack_option.len() != 10
                {
                    ttrack_option.insert(8, '0');
                }
            }

            if tpresence_option.len() != 10 {
                if tpresence_option.match_indices('-').nth(1) == Some((6, "-")) {
                    tpresence_option.insert(5, '0');
                }
                if tpresence_option.match_indices('-').nth(1) == Some((7, "-"))
                    && tpresence_option.len() != 10
                {
                    tpresence_option.insert(8, '0');
                }
            }
            // use futures::future::join_all;
            // let mut handles = vec![];

            let (tsender, treceiver) = channel();
            let (psender, preceiver) = channel();
            let (replay_sender, replay_receiver) = channel();

            let cloned_url = turl_text.clone();

            let worker = tokio::spawn(async move {
                use futures::future::join_all;
                let mut handles = vec![];
                use google_maps::prelude::*;

                if !tkey.is_empty() {
                    let google_maps_client = GoogleMapsClient::new(&tkey);
                    let directions = google_maps_client
                        .directions(
                            Location::Address(tlat_text),
                            Location::Address(tlon_text),
                            // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
                        )
                        .with_travel_mode(TravelMode::Driving)
                        .execute()
                        .await;

                    simulation(
                        directions,
                        &tkey,
                        &turl_text,
                        &ttrack_option,
                        &tpresence_option,
                        &ttrack_file,
                        &tpresence_file,
                        treceiver,
                        preceiver,
                    )
                    .await;
                } else if !tchosen_json_file.is_empty() {
                    let replay_worker = tokio::spawn(async move {
                        replay_one_file(&turl_text.clone(), &tchosen_json_file, replay_receiver)
                            .await;
                    });
                    handles.push(replay_worker);
                } else {
                    replay(
                        &turl_text.clone(),
                        &ttrack_option,
                        &"2023-02-08".to_string(),
                        &ttrack_file,
                        &"presence.json".to_string(),
                        treceiver,
                        preceiver,
                    )
                    .await;
                }

                join_all(handles).await;

                for index in 0..THREADS.len() {
                    if let Some(e) = THREADS.load(index) {
                        if e.0 == *cloned_url {
                            THREADS.store(index, None);
                        }
                    }
                }

                println!("Work Done !")
            });

            THREADS.store(
                INDEX.load(Ordering::Relaxed),
                (
                    turl_thread,
                    worker,
                    tsender,
                    psender,
                    AtomicI8::new(1),
                    replay_sender,
                ),
            );
            INDEX.store(INDEX.load(Ordering::Relaxed) + 1, Ordering::Relaxed);

            // handles.push(worker);
            // join_all(handles).await;

            println!("Shuting down the Request Handler thread!!")
        });

        return Redirect::to(uri!(index("Simulating !")));
    }
    Redirect::to(uri!(index("Didn't receive a Pong or url already is use !")))
}

fn read_json_array_from_file(file_path: &str) -> serde_json::Result<Vec<Value>> {
    dotenv().ok();
    let mut file = File::open(format!(
        "{}{}{}",
        env::var("DIR").unwrap(),
        "uploads/",
        file_path
    ))
    .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let json_value: Value = serde_json::from_str(&contents).unwrap();
    let array = json_value
        .as_array()
        .ok_or_else(|| println!("Not a JSON array"))
        .unwrap();
    Ok(array.to_vec())
}
use serde_json::{json, Value};
use std::fs::File;
use std::io::Read;

pub async fn replay_one_file(url: &String, path: &String, receiver: Receiver<()>) {
    let data_array = read_json_array_from_file(path).unwrap();

    let mut missed_data: Vec<Value> = vec![];

    let mut previous: Value = json!("");

    let client = reqwest::Client::new();

    let mut first = true;

    let mut current_status: i8 = 1;

    'outer: for json_value in data_array {
        if first {
            time::sleep(Duration::from_secs(1)).await;
            first = false;
        } else {
            let t1 = DateTime::parse_from_rfc3339(match json_value["payload"]["time"].as_str() {
                Some(e) => e,
                _ => json_value["payload"]["received_at"].as_str().unwrap(),
            })
            .unwrap();
            let t2 = DateTime::parse_from_rfc3339(match previous["payload"]["time"].as_str() {
                Some(e) => e,
                _ => previous["payload"]["received_at"].as_str().unwrap(),
            })
            .unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            for _ in 0..elapsed_seconds {
                time::sleep(Duration::from_secs(1)).await;
                match receiver.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating presence thread.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        previous = json_value.clone();

        println!("sending (Replay one file)...");

        if missed_data.len() == 10 {
            break 'outer;
        }

        let builder = {
            if current_status == 1 {
                client.post(url).json(&vec![&json_value])
            } else {
                client.post(url).json(&missed_data)
            }
        };
        let res = builder.send().await;

        match res {
            Ok(_a) => match current_status {
                1 => continue,
                0 => {
                    missed_data = vec![];
                    current_status = 1;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                _ => continue,
            },
            Err(_err) => match current_status {
                1 => {
                    missed_data.push(json_value);
                    current_status = 0;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                0 => missed_data.push(json_value),
                _ => continue,
            },
        }
        for index in 0..THREADS.len() {
            if let Some(e) = THREADS.load(index) {
                if e.0 == *url {
                    e.4.store(current_status, Ordering::Relaxed);
                }
            }
        }
    }
    println!("Shuting down the one file replay Handler thread!!")
}

async fn simulation(
    directions: GDirection,
    api_key: &String,
    url: &String,
    track_option: &String,
    presence_option: &String,
    track_file: &String,
    presence_file: &String,
    treceiver: Receiver<()>,
    preceiver: Receiver<()>,
) {
    let client = get_client().await.unwrap();

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
        simulate_presences(&url_clone, presences, presence_client, preceiver).await;
    });
    let api_key_clone = api_key.clone();

    let track_worker = tokio::spawn(async move {
        simulate_tracks(
            directions,
            api_key_clone,
            &url_clonee,
            tracks,
            track_client,
            treceiver,
        )
        .await;
    });

    handles.push(presence_worker);

    handles.push(track_worker);

    join_all(handles).await;

    for index in 0..THREADS.len() {
        if let Some(e) = THREADS.load(index) {
            if e.0 == *url {
                THREADS.store(index, None);
            }
        }
    }

    println!("Shuting down the simulation thread Handler !!")
}

use rust_decimal::prelude::*;
use std::f64::consts::PI;

trait ToRadians {
    fn to_radians(self) -> Self;
}

impl ToRadians for f64 {
    fn to_radians(self) -> f64 {
        self * PI / 180.0
    }
}

// fn float64_size_4_to_base64(f64_num: f64) -> String {
//     use base64::{engine::general_purpose, Engine as _};
//     let decimal_num = ((f64_num.round() as f32).abs() * 1000.0).round() as i32;
//     let sign_bit = if f64_num < 0.0 { 0x80 } else { 0x00 };
//     let decimal_bytes: [u8; 4] = decimal_num.to_be_bytes();
//     let mut bytes = [0u8; 4];
//     bytes[0] = sign_bit | decimal_bytes[0];
//     bytes[1] = decimal_bytes[1];
//     bytes[2] = decimal_bytes[2];
//     bytes[3] = decimal_bytes[3];
//     println!("{}", general_purpose::STANDARD.encode(&bytes));
//     general_purpose::STANDARD.encode(&bytes)
// }

fn float64_to_base64(f64_num: f64) -> String {
    let fixed_point_value = (f64_num * 10000.0).round();
    let signed_integer = fixed_point_value as i32;
    let byte_array: [u8; 4] = unsafe { mem::transmute(signed_integer.to_le()) };
    base64::encode(&byte_array)
}

fn int_to_base64(value: i32) -> String {
    let bytes = value.to_be_bytes();
    base64::encode_config(&bytes, base64::STANDARD)
}

fn bool_to_base64(boolean_value: bool) -> String {
    let byte_value: &[u8] = if boolean_value { b"\x01" } else { b"\x00" };
    base64::encode(&byte_value)
}

async fn simulate_tracks(
    directions: GDirection,
    api_key: String,
    url: &String,
    tracks: Vec<Document>,
    client: reqwest::Client,
    receiver: Receiver<()>,
) {
    use google_maps::prelude::*;

    let json_data = &directions.unwrap().routes[0].legs[0];

    let mut index = 0;

    let mut tracks_array: Vec<Req<Tracks>> = vec![];

    let mut current_status: i8 = 1;

    let mut first = true;

    let mut start_time = Instant::now();

    'outer: for step in &json_data.steps {
        use geo::CoordsIter;

        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();

        let gps_speed: f64 = ((step.distance.value as f64
            / (step.duration.value.num_minutes() + (step.duration.value.num_hours() * 360))
                as f64)
            / 3.6)
            * 1000 as f64
            / 1.852;

        let mut start = (
            step.start_location.lat.to_f64().unwrap(),
            step.start_location.lng.to_f64().unwrap(),
        );

        for coord in &result {
            if first {
                first = false;
                thread::sleep(std::time::Duration::from_secs(5));
            } else {
                thread::sleep(std::time::Duration::from_secs_f32(
                    ((step.duration.value.num_minutes() * 60)
                        + step.duration.value.num_seconds()
                        + (step.duration.value.num_hours() * 360)) as f32
                        / CoordsIter::coords_count(&result) as f32,
                ));
            }
            match receiver.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating tracks thread.");
                    break 'outer;
                }
                Err(TryRecvError::Empty) => {}
            }

            let end = (coord.y, coord.x);

            let direction = (end.1 - start.1).atan2(end.0 - start.0) * 180.0 / PI;

            start = (coord.y, coord.x);

            let mut track = Tracks {
                id: tracks[index].get_i64("id").unwrap(),
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
                connection_id: tracks[index].get_i64("connection_id").unwrap(),
                index: tracks[index].get_i64("index").unwrap(),
                fields: Fields::from(tracks[index].get_document("fields").unwrap()),
                url: Some(tracks[index].get_str("url").unwrap().to_string()),
            };

            let alt_url = format!(
                "https://maps.googleapis.com/maps/api/elevation/json?locations={},{}&key={}",
                coord.y, coord.x, api_key
            );

            let response = reqwest::get(&alt_url)
                .await
                .unwrap()
                .json::<ElevationResponse>()
                .await
                .unwrap();

            if response.status == ElevationStatus::Ok {
                let result = &response.results.unwrap()[0];
                track.fields.gps_altitude = Some(Base64 {
                    b64_value: Some(float64_to_base64(result.elevation)),
                });
            }

            track.fields.gps_speed = Some(Base64 {
                b64_value: Some(float64_to_base64(gps_speed)),
            });
            track.fields.gps_dir = Some(Base64 {
                b64_value: Some(float64_to_base64(direction)),
            });
            track.fields.dio_ignition = Some(Base64 {
                b64_value: Some(bool_to_base64(true)),
            });
            track.fields.obd_connected_protocol = Some(Base64 {
                b64_value: Some(int_to_base64(6)),
            });
            track.fields.mdi_journey_state = Some(Base64 {
                b64_value: Some(bool_to_base64(true)),
            });

            tracks_array.push(Req {
                meta: Eveent {
                    event: "track".to_string(),
                    account: "municio".to_string(),
                },
                payload: track.clone(),
            });

            if start_time.elapsed() > std::time::Duration::from_secs(11) {
                println!("sending tracks...");

                let res = client.post(url).json(&tracks_array).send().await;

                index += 1;

                start_time = Instant::now();

                match res {
                    Ok(_a) => match current_status {
                        1 => {
                            tracks_array = vec![];
                            continue;
                        }
                        0 => {
                            tracks_array = vec![];
                            current_status = 1;
                            for index in 0..THREADS.len() {
                                if let Some(e) = THREADS.load(index) {
                                    if e.0 == *url {
                                        e.4.store(current_status, Ordering::Relaxed);
                                    }
                                }
                            }
                        }
                        _ => continue,
                    },
                    Err(_err) => match current_status {
                        1 => {
                            current_status = 0;
                            if tracks_array.len() == 20 {
                                break 'outer;
                            }
                            for index in 0..THREADS.len() {
                                if let Some(e) = THREADS.load(index) {
                                    if e.0 == *url {
                                        e.4.store(current_status, Ordering::Relaxed);
                                    }
                                }
                            }
                        }
                        0 => {
                            if tracks_array.len() == 20 {
                                break 'outer;
                            }
                        }
                        _ => continue,
                    },
                }
            }
        }
    }
    println!("Shuting down tracks simulation the Handler thread!!")
}

async fn simulate_presences(
    url: &String,
    presences: Vec<Document>,
    presence_client: reqwest::Client,
    receiver: Receiver<()>,
) {
    let mut presences_array: Vec<Req<Presence>> = vec![];

    let mut old_pres: Document = Document::new();

    let mut first = true;

    let mut current_status: i8 = 1;

    'outer: for presence in presences {
        if first {
            time::sleep(Duration::from_secs_f32(0.1)).await;
            first = false;
        } else {
            let t1 = DateTime::parse_from_rfc3339(presence.get_str("time").unwrap()).unwrap();
            let t2 = DateTime::parse_from_rfc3339(old_pres.get_str("time").unwrap()).unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            for _ in 0..elapsed_seconds {
                time::sleep(Duration::from_secs(1)).await;
                match receiver.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating presence.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        old_pres = presence.clone();

        println!("sending presence...");

        if presences_array.len() == 10 {
            break 'outer;
        }

        let builder = {
            if current_status == 1 {
                presence_client.post(url).json(&vec![Req {
                    meta: Eveent {
                        event: "presence".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Presence {
                        id: presence.get_i64("id").unwrap(),
                        connection_id: presence.get_i64("connection_id").unwrap(),
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
                        time: Some((Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    },
                }])
            } else {
                presence_client.post(url).json(&presences_array)
            }
        };
        let res = builder.send().await;

        match res {
            Ok(_a) => match current_status {
                1 => continue,
                0 => {
                    presences_array = vec![];
                    current_status = 1;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                _ => continue,
            },
            Err(_err) => match current_status {
                1 => {
                    presences_array.push(Req {
                        meta: Eveent {
                            event: "presence".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Presence {
                            id: presence.get_i64("id").unwrap(),
                            connection_id: presence.get_i64("connection_id").unwrap(),
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
                            time: Some((Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                        },
                    });
                    current_status = 0;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                0 => presences_array.push(Req {
                    meta: Eveent {
                        event: "presence".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Presence {
                        id: presence.get_i64("id").unwrap(),
                        connection_id: presence.get_i64("connection_id").unwrap(),
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
                        time: Some((Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    },
                }),
                _ => continue,
            },
        }
        for index in 0..THREADS.len() {
            if let Some(e) = THREADS.load(index) {
                if e.0 == *url {
                    e.4.store(current_status, Ordering::Relaxed);
                }
            }
        }
    }
    println!("Shuting down presence simulation the Handler thread!!")
}

async fn replay(
    url: &String,
    track_option: &String,
    presence_option: &String,
    track_file: &String,
    presence_file: &String,
    treceiver: Receiver<()>,
    preceiver: Receiver<()>,
) {
    let client = get_client().await.unwrap();

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

    for index in 0..THREADS.len() {
        if let Some(e) = THREADS.load(index) {
            if e.0 == *url {
                THREADS.store(index, None);
            }
        }
    }

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

    let mut current_status: i8 = 1;

    'outer: for track in tracks {
        if first {
            time::sleep(Duration::from_secs(1)).await;
            first = false;
        } else {
            let t1 = DateTime::parse_from_rfc3339(track.get_str("recorded_at").unwrap()).unwrap();
            let t2 =
                DateTime::parse_from_rfc3339(old_track.get_str("recorded_at").unwrap()).unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            for _ in 0..elapsed_seconds {
                time::sleep(Duration::from_secs(1)).await;
                match receiver.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating tracks thread.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        old_track = track.clone();

        let location = track.get_array("location").unwrap_or(&a);
        let loc = track.get_array("loc").unwrap_or(&a);

        println!("sending replay tracks...");

        if tracks_array.len() == 10 {
            break 'outer;
        }

        let builder = {
            if current_status == 1 {
                track_client.post(url).json(&vec![Req {
                    meta: Eveent {
                        event: "track".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Tracks {
                        id: track.get_i64("id").unwrap(),
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
                        connection_id: track.get_i64("connection_id").unwrap(),
                        index: track.get_i64("index").unwrap(),
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
            Ok(_a) => match current_status {
                1 => continue,
                0 => {
                    tracks_array = vec![];
                    current_status = 1;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                _ => continue,
            },
            Err(_err) => match current_status {
                1 => {
                    tracks_array.push(Req {
                        meta: Eveent {
                            event: "track".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Tracks {
                            id: track.get_i64("id").unwrap(),
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
                            connection_id: track.get_i64("connection_id").unwrap(),
                            index: track.get_i64("index").unwrap(),
                            fields: Fields::from(track.get_document("fields").unwrap()),
                            url: Some(track.get_str("url").unwrap().to_string()),
                        },
                    });
                    current_status = 0;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                0 => tracks_array.push(Req {
                    meta: Eveent {
                        event: "track".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Tracks {
                        id: track.get_i64("id").unwrap(),
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
                        connection_id: track.get_i64("connection_id").unwrap(),
                        index: track.get_i64("index").unwrap(),
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

    let mut current_status: i8 = 1;

    'outer: for presence in presences {
        if first {
            time::sleep(Duration::from_secs(1)).await;
            first = false;
        } else {
            let t1 = DateTime::parse_from_rfc3339(presence.get_str("time").unwrap()).unwrap();
            let t2 = DateTime::parse_from_rfc3339(old_pres.get_str("time").unwrap()).unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            for _ in 0..elapsed_seconds {
                time::sleep(Duration::from_secs(1)).await;
                match receiver.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Terminating presence thread.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        old_pres = presence.clone();

        println!("sending presence...");

        if presences_array.len() == 10 {
            break 'outer;
        }

        let builder = {
            if current_status == 1 {
                presence_client.post(url).json(&vec![Req {
                    meta: Eveent {
                        event: "presence".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Presence {
                        id: presence.get_i64("id").unwrap(),
                        connection_id: presence.get_i64("connection_id").unwrap(),
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
            Ok(_a) => match current_status {
                1 => continue,
                0 => {
                    presences_array = vec![];
                    current_status = 1;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                _ => continue,
            },
            Err(_err) => match current_status {
                1 => {
                    presences_array.push(Req {
                        meta: Eveent {
                            event: "presence".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: Presence {
                            id: presence.get_i64("id").unwrap(),
                            connection_id: presence.get_i64("connection_id").unwrap(),
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
                    current_status = 0;
                    for index in 0..THREADS.len() {
                        if let Some(e) = THREADS.load(index) {
                            if e.0 == *url {
                                e.4.store(current_status, Ordering::Relaxed);
                            }
                        }
                    }
                }
                0 => presences_array.push(Req {
                    meta: Eveent {
                        event: "presence".to_string(),
                        account: "municio".to_string(),
                    },
                    payload: Presence {
                        id: presence.get_i64("id").unwrap(),
                        connection_id: presence.get_i64("connection_id").unwrap(),
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
        for index in 0..THREADS.len() {
            if let Some(e) = THREADS.load(index) {
                if e.0 == *url {
                    e.4.store(current_status, Ordering::Relaxed);
                }
            }
        }
    }
    println!("Shuting down presence replay the Handler thread!!")
}

#[post("/abort", data = "<url>")]
pub fn abort_thread(url: String) {
    for index in 0..THREADS.len() {
        if let Some(e) = THREADS.load(index) {
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
                match e.5.send(()) {
                    Ok(_e) => println!("Terminating replay one file signal !"),
                    Err(e) => println!("{:?}", e),
                }
                THREADS.store(index, None);
            }
        }
    }
}

#[get("/events")]
pub fn stream() -> EventStream![] {
    use serde_json::Map;
    EventStream! {
        let mut interval = time::interval(Duration::from_secs(2));

        loop {
            let mut map = Map::new();
            let mut threads = Map::new();
            for index in 0..THREADS.len(){
                if let Some(e) = THREADS.load(index) {
                    threads.insert(e.0.to_string(), e.4.load(Ordering::Relaxed).into());
                }
            }
            map.insert("threads".to_string(),threads.into());
            map.insert("record".to_string(),RECORD.load(Ordering::Relaxed).into());
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

    dotenv().ok();
    let path = format!("{}{}", env::var("DIR").unwrap(), "uploads/");
    let dir_entries = fs::read_dir(path).unwrap();

    // Use the `map()` function to extract the file names from the directory entries
    let file_names = dir_entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("trip_"))
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    Template::render(
        "index",
        context! {msg:msg,presence_dates:presence_dates,track_dates:track_dates,json_data:file_names},
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

    dotenv().ok();
    let path = format!("{}{}", env::var("DIR").unwrap(), "uploads/");
    let dir_entries = fs::read_dir(path).unwrap();

    // Use the `map()` function to extract the file names from the directory entries
    let file_names = dir_entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("trip_"))
        .map(|entry| entry.file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    Template::render(
        "index",
        context! {msg:"",presence_dates:presence_dates,track_dates:track_dates,json_data:file_names},
    )
}

async fn get_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

// async fn store_tracks(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
//     dotenv().ok();

//     let data = match file {
//         Some(file_name) => {
//             fs::read_to_string("./uploads/".to_owned() + file_name).expect("Unable to read file")
//         }
//         None => "".to_owned(),
//     };

//     let json_data: Vec<Tracks> = serde_json::from_str(&data).expect("Unable to read file");

//     let client = get_client().await.unwrap();

//     let collection = client.database("munic").collection("tracks");

//     for track in json_data {
//         match collection.insert_one(track, None).await {
//             Ok(_e) => continue,
//             Err(_e) => println!("track storage panic !!"),
//         }
//     }
//     Ok(())
// }
// async fn update_presence(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let client = get_client().await.unwrap();

//     let collection: Collection<Tracks> = client.database("munic").collection("presences");

//     let filter = doc! {"file":{"$exists":false}};
//     let update = doc! {"$set": {"file":file}};
//     collection.update_many(filter, update, None).await.unwrap();
//     Ok(())
// }

// async fn update_tracks(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let client = get_client().await.unwrap();

//     let collection: Collection<Tracks> = client.database("munic").collection("tracks");

//     let filter = doc! {"file":{"$exists":false}};
//     let update = doc! {"$set": {"file":file}};
//     collection.update_many(filter, update, None).await.unwrap();
//     Ok(())
// }
// async fn store_presence(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let data = match file {
//         Some(file_name) => {
//             fs::read_to_string("./uploads/".to_owned() + file_name).expect("Unable to read file")
//         }
//         None => "".to_owned(),
//     };

//     let json_data: Vec<Presence> = serde_json::from_str(&data).expect("Unable to read file");

//     let client = get_client().await.unwrap();

//     let collection = client.database("munic").collection("presences");

//     for presence in json_data {
//         match collection.insert_one(presence, None).await {
//             Ok(_e) => continue,
//             Err(_e) => panic!("presence storage panic !!"),
//         }
//     }
//     Ok(())
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
