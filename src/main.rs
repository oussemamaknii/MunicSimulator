extern crate rocket;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
pub mod models;
pub mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![services::create_post])
        .mount("/", routes![services::list])
        .attach(Template::fairing())
}
