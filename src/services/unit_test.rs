use std::thread;
use std::time::Duration;

use rocket::post;
use rocket::serde::json::Json;

#[post("/testint", format = "json", data = "<json_data>")]
pub fn test_int(json_data: Json<serde_json::Value>) {
    use rand::Rng;
    let json_value = json_data.into_inner();

    if let serde_json::Value::Object(obj) = json_value {
        let mut rng = rand::thread_rng();
        let mut current_value = rng.gen_range(
            obj["min"].as_str().unwrap().parse::<i16>().unwrap()
                ..=obj["max"].as_str().unwrap().parse::<i16>().unwrap(),
        );
        info!(target: "special","Initial value: {}", current_value);

        loop {
            thread::sleep(Duration::from_secs(1));
            let deviation_value = rng.gen_range(
                -obj["deviation"].as_str().unwrap().parse::<i16>().unwrap()
                    ..=obj["deviation"].as_str().unwrap().parse::<i16>().unwrap(),
            );
            current_value += deviation_value;

            // Vérifier les limites min/max
            current_value = current_value
                .max(obj["min"].as_str().unwrap().parse::<i16>().unwrap())
                .min(obj["max"].as_str().unwrap().parse::<i16>().unwrap());

            info!(target: "special","Next value: {}", current_value);
        }
    }
}
