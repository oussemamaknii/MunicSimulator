use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use rand::Rng;
use serde_json::Value;
use std::{
    env,
    error::Error,
    fs::File,
    io::Read,
    time::{Duration, Instant},
};
use url::Url;

pub(crate) fn parse_instant_from_string(date_string: Option<&str>) -> Option<Instant> {
    match date_string {
        Some(date_string) => {
            let date_time = DateTime::parse_from_rfc3339(date_string);
            if let Ok(date_time) = date_time {
                let now = Utc::now();
                let now_instant = Instant::now();

                let duration_since_epoch =
                    date_time.with_timezone(&Utc).timestamp() - now.timestamp();
                let instant = if duration_since_epoch < 0 {
                    now_instant - Duration::from_secs(duration_since_epoch.abs() as u64)
                } else {
                    now_instant + Duration::from_secs(duration_since_epoch.abs() as u64)
                };
                Some(instant)
            } else {
                None
            }
        }
        None => None,
    }
}

pub(crate) fn generate_string(length: i64) -> String {
    let characters = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()_+~`|}{[]\\:;?><,./-=";
    let mut rng = rand::thread_rng();
    let result: String = (0..length)
        .map(|_| {
            characters
                .chars()
                .nth(rng.gen_range(0..characters.len()))
                .unwrap()
        })
        .collect();
    result
}

pub(crate) fn get_random_string_element(array: &Vec<Value>) -> Option<String> {
    let mut rng = rand::thread_rng();
    if let Some(Value::String(s)) = array.get(rng.gen_range(0..array.len())) {
        Some(s.to_string())
    } else {
        None
    }
}

pub(crate) fn get_int_value(
    mut min: i16,
    mut max: i16,
    deviation: i16,
    first: &mut bool,
    last_int_value: &mut i16,
) -> i16 {
    let mut rng = rand::thread_rng();
    let current_value = rng.gen_range(min..=max);
    if *first {
        *last_int_value = current_value;
        *first = false;
        current_value
    } else {
        let deviation_value = rng.gen_range(-deviation..=deviation);
        *last_int_value += deviation_value;
        *last_int_value = *last_int_value.max(&mut min).min(&mut max);
        *last_int_value
    }
}

pub(crate) async fn get_client() -> Result<Client, Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

pub(crate) fn ping_server(url: String) -> bool {
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

pub(crate) fn float64_to_base64(f64_num: f64) -> String {
    let fixed_point_value = (f64_num * 10000.0).round();
    let signed_integer = fixed_point_value as i32;
    let byte_array: [u8; 4] = signed_integer.to_le().to_ne_bytes();
    base64::encode(byte_array)
}

pub(crate) fn int_to_base64(value: i32) -> String {
    let bytes = value.to_be_bytes();
    base64::encode_config(bytes, base64::STANDARD)
}

pub(crate) fn bool_to_base64(boolean_value: bool) -> String {
    let byte_value: &[u8] = if boolean_value { b"\x01" } else { b"\x00" };
    base64::encode(byte_value)
}

pub(crate) fn read_json_array_from_file(file_path: &str) -> serde_json::Result<Vec<Value>> {
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
