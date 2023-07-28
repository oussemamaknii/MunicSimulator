//service/mod.rs

extern crate chrono;
extern crate rocket;
use atomic_array::AtomicOptionRefArray;
use bson::{doc, Bson, Document};
use chrono::{DateTime, Local, Utc};
use rand::Rng;
pub mod unit_test;

// use mongodb::Collection;
mod utils;
use crate::models::{
    presences::Presence, tracks::Base64, tracks::Fields, tracks::Tracks, Eveent, Req,
};
use core::sync::atomic::{AtomicUsize, Ordering};
use dotenvy::dotenv;
use google_maps::prelude::DirectionsResponse;
use lazy_static::lazy_static;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::uri;
use rocket::Data;
use rocket::{get, post};
use rocket_dyn_templates::{context, Template};
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use rust_decimal::prelude::*;
use serde_json::{json, to_value, Value};
use std::f64::consts::PI;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::AtomicI8;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{env, thread};
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

lazy_static! {
    static ref THREADS: AtomicOptionRefArray<(
        String, // url
        JoinHandle<()>, // main thread
        Sender<()>, // track sender
        Sender<()>, // presence sender
        AtomicI8, // request status
        Sender<()>, //replay sender
        Mutex<String>, // request err message
        Mutex<String>, // request ok message
        Mutex<String>, // timestamp string
    )> = AtomicOptionRefArray::new(env::var("THREADS_NBR").unwrap().parse::<usize>().unwrap());
}

// indexing how many request threads we got
static INDEX: AtomicUsize = AtomicUsize::new(0);

static START: AtomicUsize = AtomicUsize::new(0);

// a stitc variable to know when to start recording requests coming from munic's notif server
static RECORD: AtomicUsize = AtomicUsize::new(0);

// change record state to start/stop recording
#[post("/record")]
pub fn record() {
    if RECORD.load(Ordering::Relaxed) == 0 {
        RECORD.store(1, Ordering::Relaxed);
    } else {
        RECORD.store(0, Ordering::Relaxed);
    }
}

#[post("/config", format = "json", data = "<json_data>")]
pub fn config(json_data: Json<serde_json::Value>) -> rocket::response::status::Custom<String> {
    dotenv().ok();
    info!(target: "special","configuring !");
    if let Ok(wd) = env::var("DIR") {
        if wd != json_data["wd"].as_str().unwrap().to_owned() {
            env::set_var("DIR", json_data["wd"].as_str().unwrap().to_owned())
        }
    };
    if let Ok(shutdown) = env::var("SHUTDOWN") {
        if shutdown != json_data["shutdown"].as_str().unwrap().to_owned() {
            env::set_var(
                "SHUTDOWN",
                json_data["shutdown"].as_str().unwrap().to_owned(),
            )
        }
    };
    if let Ok(imei) = env::var("IMEI") {
        if imei != json_data["imei"].as_str().unwrap().to_owned() {
            env::set_var("IMEI", json_data["imei"].as_str().unwrap().to_owned())
        }
    };
    if let Ok(imei) = env::var("THREADS_NBR") {
        if imei != json_data["threads"].as_str().unwrap().to_owned() {
            env::set_var(
                "THREADS_NBR",
                json_data["threads"].as_str().unwrap().to_owned(),
            )
        }
    };

    rocket::response::status::Custom(Status::Ok, "Processed JSON data successfully".to_string())
}

// notif is a request handling function that stores json data sent from munic's notif server into trips files found in uploads directory
#[post("/", format = "json", data = "<json_data>")]
pub async fn notif(json_data: Json<serde_json::Value>) -> rocket::response::status::Custom<String> {
    // check if the record button is active
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
        let file_name: String = if file_count == 0 {
            format!("{}{}_{}{}", prefix, file_count + 1, date, file_extension)
        } else {
            format!("{}{}_{}{}", prefix, file_count, date, file_extension)
        };

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
                            if let Some(e) = json_obj["payload"].get("type") {
                                if e.as_str().unwrap() == "connect" {
                                    file.write_all(b"[").expect("failed to write to file");
                                    START.store(1, Ordering::Relaxed);
                                } else if e.as_str().unwrap() == "disconnect" {
                                    let mut byte_array = serde_json::to_vec(&json_obj).unwrap();
                                    byte_array.push(b']');

                                    file.write_all(&byte_array[..])
                                        .expect("failed to write to file");
                                    START.store(0, Ordering::Relaxed);
                                }
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
                                if let Some(e) = json_obj["payload"].get("type") {
                                    if e.as_str().unwrap() == "connect" {
                                        new_file.write_all(b"[").expect("failed to write to file");
                                        START.store(1, Ordering::Relaxed);
                                    } else if e.as_str().unwrap() == "disconnect" {
                                        let mut byte_array = serde_json::to_vec(&json_obj).unwrap();
                                        byte_array.push(b']');

                                        new_file
                                            .write_all(&byte_array[..])
                                            .expect("failed to write to file");
                                        START.store(0, Ordering::Relaxed);
                                    }
                                }

                                if START.load(Ordering::Relaxed) == 1 {
                                    let mut byte_array = serde_json::to_vec(&json_obj).unwrap();

                                    byte_array.push(b',');

                                    new_file
                                        .write_all(&byte_array[..])
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
                                if let Some(e) = json_obj["payload"].get("type") {
                                    if e.as_str().unwrap() == "connect" {
                                        START.store(1, Ordering::Relaxed);
                                    } else if e.as_str().unwrap() == "disconnect" {
                                        let mut byte_array = serde_json::to_vec(&json_obj).unwrap();
                                        byte_array.push(b']');

                                        file.write_all(&byte_array[..])
                                            .expect("failed to write to file");
                                        START.store(0, Ordering::Relaxed);
                                    }
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
                        if let Some(e) = json_obj["payload"].get("type") {
                            if e.as_str().unwrap() == "connect" {
                                file.write_all(b"[").expect("failed to write to file");
                                START.store(1, Ordering::Relaxed);
                            } else if e.as_str().unwrap() == "disconnect" {
                                let mut byte_array = serde_json::to_vec(&json_obj).unwrap();
                                byte_array.push(b']');

                                file.write_all(&byte_array[..])
                                    .expect("failed to write to file");
                                START.store(0, Ordering::Relaxed);
                            }
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

// obivously this req handlin func is made for storing files into the uploads directory
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
        return Redirect::to(uri!(index("File stored Successfully!")));
    }
    Redirect::to(uri!(index("A problem occured storing your file !")))
}

#[post("/simulate", data = "<user_input>")]
pub async fn simulate(content_type: &ContentType, user_input: Data<'_>) -> Redirect {
    // Extracting all form fields using multi-part-form-data crate
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::text("url"),
        MultipartFormDataField::text("destination"),
        MultipartFormDataField::text("source"),
        MultipartFormDataField::text("key"),
        MultipartFormDataField::text("chosen_json_file"),
        MultipartFormDataField::text("fields_size"),
        MultipartFormDataField::text("fields_data"),
        MultipartFormDataField::file("upload_custom_field")
            .content_type_by_string(Some(mime::APPLICATION_JSON))
            .unwrap(),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, user_input, options)
        .await
        .unwrap();

    let url = multipart_form_data.texts.remove("url");
    let mut url_text: String = "".to_string();
    let mut url_thread: String = "".to_string();
    let destination = multipart_form_data.texts.remove("destination");
    let mut destination_text: String = "".to_string();
    let source = multipart_form_data.texts.remove("source");
    let mut source_text: String = "".to_string();
    let chosen_json_file = multipart_form_data.texts.remove("chosen_json_file");
    let mut chosen_json_file_text: String = "".to_string();
    let key = multipart_form_data.texts.remove("key");
    let mut key_text: String = "".to_string();
    let custom_fields = multipart_form_data.texts.remove("fields_data");
    let mut custom_fields_json: Vec<Value> = Vec::new();
    let fields_size = multipart_form_data.texts.remove("fields_size");
    let custom_field = multipart_form_data.files.get("upload_custom_field");

    if let Some(tcustom_field) = custom_field {
        let file_field = &tcustom_field[0];

        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;
        if let Ok(content) = fs::read_to_string(_path) {
            if let Ok(json_value) = serde_json::from_str::<Vec<Value>>(&content) {
                custom_fields_json = json_value;
            }
        }
    }

    if let Some(mut custom_fields) = custom_fields {
        let text_field = custom_fields.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;
        if !_text.is_empty() {
            // parsing text field text into json object
            let mut json_array: Vec<Value> =
                serde_json::from_str(&_text).expect("Failed to parse JSON array");
            if !custom_fields_json.is_empty() {
                custom_fields_json.append(&mut json_array)
            } else {
                custom_fields_json = json_array;
            }
        }
    }

    if let Some(mut fields_size) = fields_size {
        let text_field = fields_size.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;
    }
    if let Some(mut url) = url {
        let text_field = url.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        url_thread = _text.clone();
        url_text = _text;
    }
    if let Some(mut destination) = destination {
        let text_field = destination.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        destination_text = _text;
    }
    if let Some(mut source) = source {
        let text_field = source.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        source_text = _text;
    }
    if let Some(mut key) = key {
        let text_field = key.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        key_text = _text;
    }
    if let Some(mut chosen_json_file) = chosen_json_file {
        let text_field = chosen_json_file.remove(0);

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        chosen_json_file_text = _text;
    }

    for index in 0..THREADS.len() {
        if let Some(thread_ref) = THREADS.load(index) {
            if thread_ref.0 == url_text {
                return Redirect::to(uri!(index("Notification URL already exists !")));
            }
        };
    }

    // Checking if the api key is valid & at the same time getting directions for the trip
    use google_maps::prelude::*;
    let mut directions: DirectionsResponse = DirectionsResponse::from_str(r#"{
        "geocoded_waypoints" : [
           {
              "geocoder_status" : "OK",
              "place_id" : "ChIJE9on3F3HwoAR9AhGJW_fL-I",
              "types" : [ "locality", "political" ]
           },
           {
              "geocoder_status" : "OK",
              "place_id" : "ChIJIQBpAG2ahYAR_6128GcTUEo",
              "types" : [ "locality", "political" ]
           }
        ],
        "routes" : [
           {
              "bounds" : {
                 "northeast" : {
                    "lat" : 37.8273634,
                    "lng" : -118.2272819
                 },
                 "southwest" : {
                    "lat" : 34.0523706,
                    "lng" : -122.4228136
                 }
              },
              "copyrights" : "Map data \u00a92023 Google",
              "legs" : [
              ],
              "overview_polyline" : {
                 "points" : "izynEnmupUyXrDwUbAo@vDcKhLyIgE__@ybAk~@wu@__@rLml@|`Ao~@bx@so@tkAci@pm@ypAf_@sgCvl@s^ru@ki@xZen@h~@qe@lSobBx~Bm^dq@y{@ll@m|@bbAe[`t@}Dbo@yRb[uCboA_b@~yBooBh_CqgD`}CinAfpAahCv`CkeAhnAcm@te@aa@p^wWjn@oYnjAgn@n~Ace@`k@y^dLu~Ar|@wnD~~@e{C`pAqlAfpBom@~p@ibAjCud@|Ga|@aL}g@xDe[hRkg@d]}m@h]eeBf}AevBtoA{n@pr@er@xi@ue@vo@ys@rq@q}@h{Ace@pm@yr@hL{lBnAwwA~o@uo@fv@wOrbAyZ|s@ke@xVal@xPwm@lIgVf]mm@~e@uWfy@ijBvnAaiCtv@ckDoKguBxAy|@ri@ae@~r@u{@jcBs`@zo@iJzaA}WzzCqw@~aA}ZdNcZ{G_bBkeBa[iVcZ`DmhAvt@udAjt@}eBdmBuf@~a@qx@r`Bwt@tTyy@b@cbBm@aaAjk@chA`X{cFdgAyeC|d@ml@hM}a@`i@yoAr|AqnDhnDeoMxwNqp_@vmb@qx^j_k@kfDheFimEt`FieIzzIupDtfDs_DfvBw`J`fGorXlbR}`BziAclExuDq`Ef|DykJltJenDtpDinAtjA{bAvtAsxB~~DinBdhEikCrgF{fAjdBirA~jAsyDniDguDleDaeEvfDgyIjyEycFdoC{yBplAwkBvyAwoD~_DugDtuCmmD~zCg}DfiDw`Kx|IsvBvwBk}BlrCyjJj`NcfCbrDyxB|jCuiKvmHweUbtNckLpeHazFzlE_bE|_FuzDtvEm{C`hC_nEtfEidH`{Gk|DpdD{gCnyAisIh}EeaAjg@}zAlW_eB`Yo~Fn`AaoH|nAc|ItpBu}Axj@cc@bXuwA||@mgFraCgnEhuDmhExhFghDvjEwaIhaLmyDvrF}mDp~GmmIbgPe}@reBe~An`Cwo@lu@aj@zaA`RprDdMh_CbYdtBxe@~j@nS~kA|Wbw@xEzz@uTfbA`ZnmCpxAjxEj@n{BpN~rBiC|uJ}B`eLjTtcE|CbgFsd@doDnIxtBStcAxJry@~g@|_ApPfhE[naBiw@x|@mm@hoAir@x`A{_Atd@ilArV__Azw@c_@bBqd@aS}pAtz@udA~lCmJltAel@d_Aa\\|_A{QprAyz@n~Bya@hRsa@hg@yc@`oCtBpmBlXhsCdMbvBn\\~gBf`C~oCtlBteCfTfWzVkD|FnJyBzc@oHjx@_WmS"
              },
              "summary" : "I-5 N",
              "warnings" : [],
              "waypoint_order" : []
           }
        ],
        "status" : "OK"
     }"#).unwrap();

    if !key_text.is_empty() {
        let google_maps_client = GoogleMapsClient::new(&key_text);
        if let Ok(dir) = google_maps_client
            .directions(
                Location::Address(source_text),
                Location::Address(destination_text),
                // Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904).unwrap()),
            )
            .with_travel_mode(TravelMode::Driving)
            .execute()
            .await
        {
            directions = dir;
        } else {
            return Redirect::to(uri!(index(
                "Invalid API_KEY check the Documentation for setting one or Invalid source/destination coordinates !"
            )));
        }
    }

    // check if the server is online
    if utils::ping_server(url_text.clone()) {
        let _request_handler = tokio::spawn(async move {
            let (tsender, treceiver) = channel();
            let (psender, preceiver) = channel();
            let (replay_sender, replay_receiver) = channel();

            let cloned_url = url_text.clone();

            let worker = tokio::spawn(async move {
                use futures::future::join_all;
                let mut handles = vec![];

                if !key_text.is_empty() {
                    // real-time simulation
                    simulation(
                        directions,
                        &key_text,
                        &url_text,
                        custom_fields_json,
                        treceiver,
                        preceiver,
                    )
                    .await;
                } else if !chosen_json_file_text.is_empty() {
                    // replay one single file
                    if let Ok(data_array) = utils::read_json_array_from_file(&chosen_json_file_text)
                    {
                        let replay_worker = tokio::spawn(async move {
                            replay_one_file(
                                &url_text.clone(),
                                data_array,
                                &mut custom_fields_json,
                                replay_receiver,
                            )
                            .await;
                        });
                        handles.push(replay_worker);
                    } else {
                        error!(target: "special",
                            "can't read json array from file {} !",
                            chosen_json_file_text
                        );
                    }
                } else {
                    // replay multiple seprate files (tracks, pesence, messages, poke ..)
                    // waiting for confirmation to continue work on it for now it's unused feature
                    replay(
                        &url_text.clone(),
                        &"2023-02-08".to_string(),
                        &"2023-02-08".to_string(),
                        &"tracks.json".to_string(),
                        &"presence.json".to_string(),
                        treceiver,
                        preceiver,
                    )
                    .await;
                }

                // wait for the single file replay to end
                join_all(handles).await;

                // if all working threads are done we delete the main thread from our threads struct
                for index in 0..THREADS.len() {
                    if let Some(thread_ref) = THREADS.load(index) {
                        if thread_ref.0 == *cloned_url {
                            THREADS.store(index, None);
                        }
                    }
                }
                info!(target: "special","Work Done !")
            });

            // store the new Client Request Thread
            THREADS.store(
                INDEX.load(Ordering::Relaxed),
                (
                    url_thread,
                    worker,
                    tsender,
                    psender,
                    AtomicI8::new(1),
                    replay_sender,
                    Mutex::new(String::from("")),
                    Mutex::new(String::from("")),
                    Mutex::new(String::from("")),
                ),
            );

            // update the number of threads +1
            INDEX.store(INDEX.load(Ordering::Relaxed) + 1, Ordering::Relaxed);

            info!(target: "special","Shuting down the Request Handler thread!!")
        });

        return Redirect::to(uri!(index("Simulating !")));
    }
    Redirect::to(uri!(index(
        "Didn't receive a Pong make sure your server is online !"
    )))
}

pub async fn replay_one_file(
    url: &String,
    data_array: Vec<Value>,
    custom_fields: &mut Vec<Value>,
    receiver: Receiver<()>,
) {
    let mut missed_data: Vec<Value> = vec![];

    let mut previous: Value = json!("");

    let client = reqwest::Client::new();

    let mut first = true;

    let mut current_status: i8 = 1;

    let start_custom_fields_time = Instant::now();

    let mut last_int_value: i16 = 0;

    let mut first_custom_fields = true;

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
                        info!(target: "special","Terminating presence thread.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        previous = json_value.clone();

        info!(target: "special","sending (Replay one file)...");

        if missed_data.len() == 10 {
            break 'outer;
        }

        let builder = {
            if current_status == 1 {
                let mut json = vec![json_value.clone()];
                if !custom_fields.is_empty() {
                    let current_time = Instant::now();
                    add_custom_fields(
                        &mut json,
                        custom_fields,
                        start_custom_fields_time,
                        current_time,
                        &mut first_custom_fields,
                        &mut last_int_value,
                        url,
                    );
                }
                client.post(url).json(&json)
            } else {
                if !custom_fields.is_empty() {
                    let current_time = Instant::now();
                    add_custom_fields(
                        &mut missed_data,
                        custom_fields,
                        start_custom_fields_time,
                        current_time,
                        &mut first_custom_fields,
                        &mut last_int_value,
                        url,
                    );
                }
                client.post(url).json(&missed_data)
            }
        };
        let res = builder.send().await;

        match res {
            Ok(resp) => match current_status {
                1 => {
                    for index in 0..THREADS.len() {
                        if let Some(threads_ref) = THREADS.load(index) {
                            if threads_ref.0 == *url {
                                let mut ok_msg = threads_ref.7.lock().unwrap();
                                *ok_msg = resp.status().to_string();

                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                        }
                    }
                    continue;
                }
                0 => {
                    missed_data = vec![];
                    current_status = 1;
                    for index in 0..THREADS.len() {
                        if let Some(threads_ref) = THREADS.load(index) {
                            if threads_ref.0 == *url {
                                threads_ref.4.store(current_status, Ordering::Relaxed);
                                let mut ok_msg = threads_ref.7.lock().unwrap();
                                *ok_msg = resp.status().to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                        }
                    }
                }
                _ => continue,
            },
            Err(err) => match current_status {
                1 => {
                    missed_data.push(json_value);
                    current_status = 0;
                    for index in 0..THREADS.len() {
                        if let Some(threads_ref) = THREADS.load(index) {
                            if threads_ref.0 == *url {
                                threads_ref.4.store(current_status, Ordering::Relaxed);
                                let mut msg_err = threads_ref.6.lock().unwrap();
                                *msg_err = err.to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                        }
                    }
                    info!(target:"special","sent missing data !")
                }
                0 => {
                    for index in 0..THREADS.len() {
                        if let Some(threads_ref) = THREADS.load(index) {
                            if threads_ref.0 == *url {
                                let mut msg_err = threads_ref.6.lock().unwrap();
                                *msg_err = err.to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                        }
                    }
                    warn!(target: "special","missed paquets !");
                    missed_data.push(json_value)
                }
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
    info!(target: "special","Shuting down the one file replay Handler thread!!")
}

async fn simulation(
    directions: google_maps::directions::response::Response,
    api_key: &str,
    url: &String,
    mut custom_fields: Vec<Value>,
    treceiver: Receiver<()>,
    preceiver: Receiver<()>,
) {
    dotenv().ok();
    let mut file = File::open(format!(
        "{}{}",
        env::var("DIR").unwrap(),
        "uploads/refrence.json"
    ))
    .expect("Failed to open the JSON file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the JSON file");

    let json_values: Vec<Value> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let filtered_presences: Vec<Value> = json_values
        .clone()
        .into_iter()
        .filter(|event| event["meta"]["event"] == "presence")
        .collect();

    let filtered_tracks: Vec<Value> = json_values
        .into_iter()
        .filter(|event| event["meta"]["event"] == "track")
        .collect();

    let mut presences: Vec<Presence> = Vec::new();
    let mut ref_track: Tracks = serde_json::from_value::<Req<Tracks>>(filtered_tracks[0].clone())
        .unwrap()
        .payload;
    ref_track.asset = if let Ok(x) = env::var("IMEI") {
        Some(x)
    } else {
        ref_track.asset
    };

    for json_value in filtered_presences {
        if let Ok(mut req) = serde_json::from_value::<Req<Presence>>(json_value.clone()) {
            req.payload.asset = if let Ok(x) = env::var("IMEI") {
                Some(x)
            } else {
                req.payload.asset
            };
            presences.push(req.payload);
        }
    }

    let track_client = reqwest::Client::new();
    let presence_client = reqwest::Client::new();

    use futures::future::join_all;
    let mut handles = vec![];
    let url_clone = url.clone();
    let url_clonee = url.clone();

    let presence_worker = tokio::spawn(async move {
        simulate_presences(&url_clone, presences, presence_client, preceiver).await;
    });
    let api_key_clone = api_key.to_owned();

    let track_worker = tokio::spawn(async move {
        simulate_tracks(
            directions,
            api_key_clone,
            &url_clonee,
            &mut ref_track,
            track_client,
            &mut custom_fields,
            treceiver,
        )
        .await;
    });

    handles.push(track_worker);

    handles.push(presence_worker);

    join_all(handles).await;

    for index in 0..THREADS.len() {
        if let Some(e) = THREADS.load(index) {
            if e.0 == *url {
                THREADS.store(index, None);
            }
        }
    }

    info!(target: "special","Shuting down the simulation thread Handler !!")
}

async fn simulate_tracks(
    directions: DirectionsResponse,
    api_key: String,
    url: &String,
    mut ref_track: &mut Tracks,
    client: reqwest::Client,
    custom_fields: &mut Vec<Value>,
    receiver: Receiver<()>,
) {
    use google_maps::prelude::*;

    let json_data = &directions.routes[0].legs[0];

    let mut tracks_array: Vec<Req<Tracks>> = vec![];

    let mut current_status: i8 = 1;

    let mut first = true;

    let mut start_time = Instant::now();

    let start_custom_fields_time = Instant::now();

    let mut last_int_value: i16 = 0;

    let mut first_custom_fields = true;

    dotenv().ok();

    let shutdown = if let Ok(shutdwon) = env::var("SHUTDOWN") {
        if shutdwon == "true" {
            true
        } else {
            false
        }
    } else {
        false
    };

    'outer: for step in &json_data.steps {
        use geo::CoordsIter;

        let result = polyline::decode_polyline(&step.polyline.points, 5).unwrap();

        let gps_speed: f64 = ((step.distance.value as f64
            / ((step.duration.value.num_minutes() * 60)
                + step.duration.value.num_seconds()
                + (step.duration.value.num_hours() * 360)) as f64)
            * 3.6)
            * 1000_f64
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
                    info!(target: "special","Terminating tracks thread.");
                    break 'outer;
                }
                Err(TryRecvError::Empty) => {}
            }

            let end = (coord.y, coord.x);

            let direction = (end.1 - start.1).atan2(end.0 - start.0) * 180.0 / PI;

            start = (coord.y, coord.x);

            ref_track.location = Some([coord.x, coord.y]);
            ref_track.loc = Some([coord.x, coord.y]);
            ref_track.recorded_at = Some(
                (Local::now() - Duration::minutes(2))
                    .format("%Y-%m-%dT%H:%M:%SZ")
                    .to_string(),
            );
            ref_track.recorded_at_ms = Some(
                (Local::now() - Duration::minutes(2))
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    .to_string(),
            );
            ref_track.received_at = Some((Local::now()).format("%Y-%m-%dT%H:%M:%SZ").to_string());

            let alt_url = format!(
                "https://maps.googleapis.com/maps/api/elevation/json?locations={},{}&key={}",
                coord.y, coord.x, api_key
            );

            if let Ok(response) = reqwest::get(&alt_url).await {
                if let Ok(resp) = response.json::<ElevationResponse>().await {
                    let result = &resp.results.unwrap()[0];
                    ref_track.fields.gps_altitude = Some(Base64 {
                        b64_value: Some(utils::float64_to_base64(result.elevation)),
                    });
                }
            }

            ref_track.fields.gps_speed = Some(Base64 {
                b64_value: Some(utils::float64_to_base64(gps_speed)),
            });
            ref_track.fields.gps_dir = Some(Base64 {
                b64_value: Some(utils::float64_to_base64(utils::convert_negative_angle(
                    direction,
                ))),
            });
            ref_track.fields.dio_ignition = Some(Base64 {
                b64_value: Some(utils::bool_to_base64(true)),
            });
            ref_track.fields.obd_connected_protocol = Some(Base64 {
                b64_value: Some(utils::int_to_base64(6)),
            });
            ref_track.fields.mdi_journey_state = Some(Base64 {
                b64_value: Some(utils::bool_to_base64(true)),
            });

            tracks_array.push(Req {
                meta: Eveent {
                    event: "track".to_string(),
                    account: "municio".to_string(),
                },
                payload: ref_track.clone(),
            });

            if start_time.elapsed() > std::time::Duration::from_secs(11) {
                info!(target: "special","sending tracks...");

                let mut tracks_array_value: Vec<Value> = tracks_array
                    .clone()
                    .into_iter()
                    .map(|s| to_value(s).unwrap())
                    .collect();

                if !custom_fields.is_empty() {
                    let current_time = Instant::now();
                    add_custom_fields(
                        &mut tracks_array_value,
                        custom_fields,
                        start_custom_fields_time,
                        current_time,
                        &mut first_custom_fields,
                        &mut last_int_value,
                        url,
                    );
                }

                let res = client.post(url).json(&tracks_array_value).send().await;

                start_time = Instant::now();

                let mut threads_ref: Option<
                    Arc<(
                        String,
                        JoinHandle<()>,
                        Sender<()>,
                        Sender<()>,
                        AtomicI8,
                        Sender<()>,
                        Mutex<String>,
                        Mutex<String>,
                        Mutex<String>,
                    )>,
                > = None;

                for index in 0..THREADS.len() {
                    if let Some(threads_reference) = THREADS.load(index) {
                        if threads_reference.0 == *url {
                            // udate current_status because it could be changed from the presence thread
                            current_status = threads_reference.4.load(Ordering::Relaxed);
                            threads_ref = Some(threads_reference);
                            break;
                        }
                    }
                }

                match res {
                    Ok(resp) => match current_status {
                        1 => {
                            tracks_array = vec![];
                            if let Some(ref threads_ref) = threads_ref {
                                let mut ok_msg = threads_ref.7.lock().unwrap();
                                *ok_msg = resp.status().to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                            continue;
                        }
                        0 => {
                            tracks_array = vec![];
                            current_status = 1;
                            if let Some(ref threads_ref) = threads_ref {
                                threads_ref.4.store(current_status, Ordering::Relaxed);
                                let mut ok_msg = threads_ref.7.lock().unwrap();
                                *ok_msg = resp.status().to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                        }
                        _ => continue,
                    },
                    Err(err) => match current_status {
                        1 => {
                            current_status = 0;
                            if tracks_array.len() == 20 && shutdown {
                                break 'outer;
                            }

                            if let Some(ref threads_ref) = threads_ref {
                                threads_ref.4.store(current_status, Ordering::Relaxed);
                                let mut msg_err = threads_ref.6.lock().unwrap();
                                *msg_err = err.to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }
                            info!(target:"special","sent missing data !")
                        }
                        0 => {
                            if let Some(ref threads_ref) = threads_ref {
                                let mut msg_err = threads_ref.6.lock().unwrap();
                                *msg_err = err.to_string();
                                let mut time_msg = threads_ref.8.lock().unwrap();
                                *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                            }

                            warn!(target: "special","missed paquets !");

                            if tracks_array.len() == 20 && shutdown {
                                break 'outer;
                            }
                        }
                        _ => continue,
                    },
                }
            }
        }
    }
    info!(target: "special","Shuting down tracks simulation the Handler thread!!")
}

fn add_custom_fields(
    tracks_array: &mut Vec<Value>,
    custom_fields: &mut [Value],
    start_time: Instant,
    current_time: Instant,
    first: &mut bool,
    last: &mut i16,
    url: &String,
) {
    use humantime::parse_duration;

    for update_data in custom_fields.iter_mut() {
        let elapsed_time = if let Some(last_updated) =
            utils::parse_instant_from_string(update_data["last_updated"].as_str())
        {
            current_time.duration_since(last_updated)
        } else {
            current_time.duration_since(start_time)
        };
        if let Some(freq) = update_data["frequence"].as_str() {
            if let Ok(frequence) = parse_duration(freq) {
                if elapsed_time >= frequence {
                    for struct_to_update in &mut *tracks_array {
                        if let Some(object) = update_data["type"].as_object() {
                            if object.contains_key("bool") {
                                struct_to_update["payload"]["fields"]
                                    [update_data["field_name"].as_str().unwrap().to_uppercase()] = json!({"b64_value" :utils::bool_to_base64(match update_data["type"]["bool"].as_str().unwrap().parse::<bool>(){
                                    Ok(e) => e,
                                    Err(_) => {
                                        let mut rng = rand::thread_rng();
                                        rng.gen::<bool>()
                                    },
                                })});
                            }
                            if object.contains_key("int") {
                                struct_to_update["payload"]["fields"]
                                    [update_data["field_name"].as_str().unwrap().to_uppercase()] =
                                    json!(
                            {"b64_value" : utils::int_to_base64(utils::get_int_value(
                                update_data["type"]["int"]["min"]
                                    .as_i64()
                                    .unwrap() as i16,
                                update_data["type"]["int"]["max"]
                                    .as_i64()
                                    .unwrap() as i16,
                                update_data["type"]["int"]["deviation"]
                                    .as_i64()
                                    .unwrap() as i16,
                                first,
                                last,
                            ).into())});
                            }
                            if object.contains_key("string") {
                                if let Some(size) = update_data["type"]["string"]["random"].as_i64()
                                {
                                    struct_to_update["payload"]["fields"][update_data
                                        ["field_name"]
                                        .as_str()
                                        .unwrap()
                                        .to_uppercase()] = json!({
                                        "b64_value": base64::encode(utils::generate_string(size))
                                    });
                                } else if let Some(array) =
                                    update_data["type"]["string"]["array"].as_array()
                                {
                                    struct_to_update["payload"]["fields"][update_data
                                        ["field_name"]
                                        .as_str()
                                        .unwrap()
                                        .to_uppercase()] = if let Some(string) =
                                        utils::get_random_string_element(array)
                                    {
                                        json!({ "b64_value": base64::encode(string) })
                                    } else {
                                        Value::Null
                                    }
                                }
                            }
                        } else {
                            error!(target: "special","'type' is not an object");
                        }
                    }

                    let time = Utc::now();

                    let datetime_str = time.to_rfc3339();

                    update_data["last_updated"] = serde_json::to_value(Some(datetime_str))
                        .expect("error updating last updated time !!");
                }
            } else {
                error!(target: "special","can't parse frequence !");
            }
        } else {
            error!(target: "special","can't unwrap frequence !");
            abort_thread(url.to_string());
        }
    }
}

async fn simulate_presences(
    url: &String,
    presences: Vec<Presence>,
    presence_client: reqwest::Client,
    receiver: Receiver<()>,
) {
    let mut presences_array: Vec<Req<Presence>> = vec![];

    let mut old_pres = Presence::new();

    let mut first = true;

    let mut current_status: i8 = 1;

    'outer: for presence in presences {
        if first {
            time::sleep(Duration::from_secs_f32(0.1)).await;
            first = false;
        } else {
            let t1 = DateTime::parse_from_rfc3339(&presence.clone().time.unwrap()).unwrap();
            let t2 = DateTime::parse_from_rfc3339(&old_pres.time.unwrap()).unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            for _ in 0..elapsed_seconds {
                time::sleep(Duration::from_secs(1)).await;
                match receiver.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        info!(target: "special","Terminating presence.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        old_pres = presence.clone();

        info!(target: "special","sending presence...");

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
                    payload: presence.clone(),
                }])
            } else {
                presence_client.post(url).json(&presences_array)
            }
        };
        let res = builder.send().await;

        let mut threads_ref: Option<
            Arc<(
                String,
                JoinHandle<()>,
                Sender<()>,
                Sender<()>,
                AtomicI8,
                Sender<()>,
                Mutex<String>,
                Mutex<String>,
                Mutex<String>,
            )>,
        > = None;

        for index in 0..THREADS.len() {
            if let Some(threads_reference) = THREADS.load(index) {
                if threads_reference.0 == *url {
                    // udate current_status because it could be changed from the tracks thread
                    current_status = threads_reference.4.load(Ordering::Relaxed);
                    threads_ref = Some(threads_reference);
                    break;
                }
            }
        }

        match res {
            Ok(resp) => match current_status {
                1 => {
                    if let Some(ref threads_ref) = threads_ref {
                        let mut ok_msg = threads_ref.7.lock().unwrap();
                        *ok_msg = resp.status().to_string();
                        let mut time_msg = threads_ref.8.lock().unwrap();
                        *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                    }
                    continue;
                }
                0 => {
                    presences_array = vec![];
                    current_status = 1;
                    if let Some(ref threads_ref) = threads_ref {
                        threads_ref.4.store(current_status, Ordering::Relaxed);
                        let mut ok_msg = threads_ref.7.lock().unwrap();
                        *ok_msg = resp.status().to_string();
                        let mut time_msg = threads_ref.8.lock().unwrap();
                        *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                    }
                }
                _ => continue,
            },
            Err(err) => match current_status {
                1 => {
                    presences_array.push(Req {
                        meta: Eveent {
                            event: "presence".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: presence.clone(),
                    });
                    current_status = 0;
                    if let Some(ref threads_ref) = threads_ref {
                        threads_ref.4.store(current_status, Ordering::Relaxed);
                        let mut msg_err = threads_ref.6.lock().unwrap();
                        *msg_err = err.to_string();
                        let mut time_msg = threads_ref.8.lock().unwrap();
                        *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                    }
                    info!(target:"special","sent missing data !")
                }
                0 => {
                    if let Some(ref threads_ref) = threads_ref {
                        let mut msg_err = threads_ref.6.lock().unwrap();
                        *msg_err = err.to_string();
                        let mut time_msg = threads_ref.8.lock().unwrap();
                        *time_msg = Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                    }
                    warn!(target: "special","missed paquets !");
                    presences_array.push(Req {
                        meta: Eveent {
                            event: "presence".to_string(),
                            account: "municio".to_string(),
                        },
                        payload: presence.clone(),
                    })
                }
                _ => continue,
            },
        }
        if let Some(ref threads_ref) = threads_ref {
            threads_ref.4.store(current_status, Ordering::Relaxed);
        }
    }
    info!(target: "special","Shuting down presence simulation the Handler thread!!")
}

#[post("/abort", data = "<url>")]
pub fn abort_thread(url: String) {
    for index in 0..THREADS.len() {
        if let Some(e) = THREADS.load(index) {
            if e.0 == url {
                let _ = &e.1.abort();
                match e.2.send(()) {
                    Ok(_e) => info!(target: "special","Terminating tracks signal !"),
                    Err(e) => error!(target: "special","{:?}", e),
                }
                match e.3.send(()) {
                    Ok(_e) => info!(target: "special","Terminating presences signal !"),
                    Err(e) => error!(target: "special","{:?}", e),
                }
                match e.5.send(()) {
                    Ok(_e) => info!(target: "special","Terminating replay one file signal !"),
                    Err(e) => error!(target: "special","{:?}", e),
                }
                THREADS.store(index, None);
                break;
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
                    let mut info = Map::new();
                    info.insert("code".to_string(),e.4.load(Ordering::Relaxed).into());
                    let msg_err = e.6.lock().unwrap();
                    let ok_msg = e.7.lock().unwrap();
                    let timestamp = e.8.lock().unwrap();
                    info.insert("err_msg".to_string(),json!(*msg_err.clone()));
                    info.insert("ok_msg".to_string(),json!(*ok_msg.clone()));
                    info.insert("timestamp".to_string(),json!(*timestamp.clone()));

                    threads.insert(e.0.to_string(), info.into());
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
    // use mongodb::bson::doc;

    // let client = utils::get_client().await.unwrap();

    // let pres_collection: Collection<Presence> = client.database("munic").collection("presences");
    // let track_collection: Collection<Tracks> = client.database("munic").collection("tracks");

    // let trk_pipeline = vec![
    //     doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
    //     doc! {"$sort": { "date" : 1 }},
    //     doc! {"$group":  {
    //         "_id": { "file": "$file","year": { "$year": "$date" },  "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
    //     }},
    // ];
    // let pres_pipeline = vec![
    //     doc! {"$addFields": { "date": { "$toDate": "$time" } }  },
    //     doc! {"$sort": { "date" : 1 }},
    //     doc! {"$group":  {
    //         "_id": { "file": "$file","year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
    //     }},
    // ];

    // let tracks_data = track_collection
    //     .aggregate(trk_pipeline, None)
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    // let presence_data = pres_collection
    //     .aggregate(pres_pipeline, None)
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    // use futures::stream::TryStreamExt;

    // let track_dates: Vec<_> = tracks_data
    //     .try_collect()
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();
    // let presence_dates: Vec<_> = presence_data
    //     .try_collect()
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    dotenv().ok();
    let wd = if let Ok(wd) = env::var("DIR") {
        wd
    } else {
        "".to_string()
    };

    let shutdown = if let Ok(shutdown) = env::var("SHUTDOWN") {
        shutdown
    } else {
        "".to_string()
    };

    let imei: String = if let Ok(imei) = env::var("IMEI") {
        imei
    } else {
        "".to_string()
    };

    let threads_nbr: String = if let Ok(imei) = env::var("THREADS_NBR") {
        imei
    } else {
        "".to_string()
    };

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
        context! {msg:msg,json_data:file_names,wd:wd,shutdown:shutdown,imei:imei,threads_nbr:threads_nbr},
        // context! {msg:msg,presence_dates:presence_dates,track_dates:track_dates,json_data:file_names},
    )
}

#[get("/")]
pub async fn indexx() -> Template {
    // use mongodb::bson::doc;

    // let client = utils::get_client().await.unwrap();

    // let pres_collection: Collection<Presence> = client.database("munic").collection("presences");
    // let track_collection: Collection<Tracks> = client.database("munic").collection("tracks");

    // let trk_pipeline = vec![
    //     doc! {"$addFields": { "date": { "$toDate": "$recorded_at" } }  },
    //     doc! {"$sort": { "date" : 1 }},
    //     doc! {"$group":  {
    //         "_id": { "file": "$file","year": { "$year": "$date" },  "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
    //     }},
    // ];
    // let pres_pipeline = vec![
    //     doc! {"$addFields": { "date": { "$toDate": "$time" } }  },
    //     doc! {"$sort": { "date" : 1 }},
    //     doc! {"$group":  {
    //         "_id": { "file": "$file","year": { "$year": "$date" }, "month": { "$month": "$date" }, "day": { "$dayOfMonth": "$date" } }
    //     }},
    // ];

    // let tracks_data = track_collection
    //     .aggregate(trk_pipeline, None)
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    // let presence_data = pres_collection
    //     .aggregate(pres_pipeline, None)
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    // use futures::stream::TryStreamExt;

    // let track_dates: Vec<_> = tracks_data
    //     .try_collect()
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    // let presence_dates: Vec<_> = presence_data
    //     .try_collect()
    //     .await
    //     .map_err(|e| println!("{}", e))
    //     .unwrap();

    dotenv().ok();
    let wd = if let Ok(wd) = env::var("DIR") {
        wd
    } else {
        "".to_string()
    };

    let shutdown = if let Ok(shutdown) = env::var("SHUTDOWN") {
        shutdown
    } else {
        "".to_string()
    };

    let imei: String = if let Ok(imei) = env::var("IMEI") {
        imei
    } else {
        "".to_string()
    };

    let threads_nbr: String = if let Ok(imei) = env::var("THREADS_NBR") {
        imei
    } else {
        "".to_string()
    };

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
        context! {msg:"",json_data:file_names,wd:wd,shutdown:shutdown,imei:imei,threads_nbr:threads_nbr},
        // context! {msg:"",presence_dates:presence_dates,track_dates:track_dates,json_data:file_names},
    )
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
    let client = utils::get_client().await.unwrap();

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
        .map_err(|e| error!(target: "special","{}", e))
        .unwrap();
    let presence_data = presence_collection
        .aggregate(ppipeline, None)
        .await
        .map_err(|e| error!(target: "special","{}", e))
        .unwrap();

    use futures::stream::TryStreamExt;

    let tracks: Vec<_> = tracks_data
        .try_collect()
        .await
        .map_err(|e| error!(target: "special","{}", e))
        .unwrap();
    let presences: Vec<_> = presence_data
        .try_collect()
        .await
        .map_err(|e| error!(target: "special","{}", e))
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

    info!(target: "special","Shuting down replay the Handler thread!!")
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
            let t1 = DateTime::parse_from_rfc3339(track.get_str("received_at").unwrap()).unwrap();
            let t2 =
                DateTime::parse_from_rfc3339(old_track.get_str("received_at").unwrap()).unwrap();

            let elapsed_seconds = t1.timestamp() - t2.timestamp();

            for _ in 0..elapsed_seconds {
                time::sleep(Duration::from_secs(1)).await;
                match receiver.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        info!(target: "special","Terminating tracks thread.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        old_track = track.clone();

        let location = track.get_array("location").unwrap_or(&a);
        let loc = track.get_array("loc").unwrap_or(&a);

        info!(target: "special","sending replay tracks...");

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

        let mut threads_ref: Option<
            Arc<(
                String,
                JoinHandle<()>,
                Sender<()>,
                Sender<()>,
                AtomicI8,
                Sender<()>,
                Mutex<String>,
                Mutex<String>,
                Mutex<String>,
            )>,
        > = None;

        for index in 0..THREADS.len() {
            if let Some(threads_reference) = THREADS.load(index) {
                if threads_reference.0 == *url {
                    // udate current_status because it could be changed from the presence thread
                    current_status = threads_reference.4.load(Ordering::Relaxed);
                    threads_ref = Some(threads_reference);
                    break;
                }
            }
        }

        match res {
            Ok(_a) => match current_status {
                1 => continue,
                0 => {
                    tracks_array = vec![];
                    current_status = 1;
                    if let Some(ref threads_ref) = threads_ref {
                        threads_ref.4.store(current_status, Ordering::Relaxed);
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
                    if let Some(ref threads_ref) = threads_ref {
                        threads_ref.4.store(current_status, Ordering::Relaxed);
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
    info!(target: "special","Shuting down tracks replay the Handler thread!!")
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
                        info!(target: "special","Terminating presence thread.");
                        break 'outer;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }

        old_pres = presence.clone();

        info!(target: "special","sending presence...");

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

        let mut threads_ref: Option<
            Arc<(
                String,
                JoinHandle<()>,
                Sender<()>,
                Sender<()>,
                AtomicI8,
                Sender<()>,
                Mutex<String>,
                Mutex<String>,
                Mutex<String>,
            )>,
        > = None;

        for index in 0..THREADS.len() {
            if let Some(threads_reference) = THREADS.load(index) {
                if threads_reference.0 == *url {
                    // udate current_status because it could be changed from the presence thread
                    current_status = threads_reference.4.load(Ordering::Relaxed);
                    threads_ref = Some(threads_reference);
                    break;
                }
            }
        }
        match res {
            Ok(_a) => match current_status {
                1 => continue,
                0 => {
                    presences_array = vec![];
                    current_status = 1;
                    if let Some(ref threads_ref) = threads_ref {
                        threads_ref.4.store(current_status, Ordering::Relaxed);
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
                    if let Some(ref threads_ref) = threads_ref {
                        threads_ref.4.store(current_status, Ordering::Relaxed);
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
        if let Some(ref threads_ref) = threads_ref {
            threads_ref.4.store(current_status, Ordering::Relaxed);
        }
    }
    info!(target: "special","Shuting down presence replay the Handler thread!!")
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

//     let client = utils::get_client().await.unwrap();

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
//     let client = utils::get_client().await.unwrap();

//     let collection: Collection<Tracks> = client.database("munic").collection("presences");

//     let filter = doc! {"file":{"$exists":false}};
//     let update = doc! {"$set": {"file":file}};
//     collection.update_many(filter, update, None).await.unwrap();
//     Ok(())
// }

// async fn update_tracks(file: &Option<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let client = utils::get_client().await.unwrap();

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

//     let client = utils::get_client().await.unwrap();

//     let collection = client.database("munic").collection("presences");

//     for presence in json_data {
//         match collection.insert_one(presence, None).await {
//             Ok(_e) => continue,
//             Err(_e) => panic!("presence storage panic !!"),
//         }
//     }
//     Ok(())
// }

trait ToRadians {
    fn to_radians(self) -> Self;
}

impl ToRadians for f64 {
    fn to_radians(self) -> f64 {
        self * PI / 180.0
    }
}
