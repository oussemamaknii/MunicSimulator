extern crate rocket;
use rocket::{fs::FileServer, launch, routes};
use rocket_dyn_templates::Template;
pub mod models;
mod services;

#[launch]
fn rocket() -> _ {
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
                services::abort_thread
            ],
        )
        .attach(Template::fairing())
}
