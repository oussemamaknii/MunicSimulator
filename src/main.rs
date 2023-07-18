extern crate rocket;
use dotenvy::dotenv;
use rocket::{fs::FileServer, launch, routes};
use rocket_dyn_templates::Template;
pub mod models;
mod services;
#[macro_use]
extern crate log;
extern crate env_logger;
use log4rs;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    log4rs::init_file("src/logging_config.yaml", Default::default()).unwrap();

    rocket::build()
        .mount(
            "/public",
            FileServer::from("C:/Users/makni_o/Documents/MunicSimulator/static"),
        )
        .mount(
            "/",
            routes![
                services::simulate,
                services::index,
                services::indexx,
                services::upload,
                services::notif,
                services::stream,
                services::record,
                services::config,
                services::unit_test::test_int,
                services::abort_thread
            ],
        )
        .attach(Template::fairing())
}
