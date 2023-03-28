extern crate rocket;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
pub mod models;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                services::simulate,
                services::index,
                services::indexx,
                services::store_p,
                services::store_t,
                services::test,
                services::stream
            ],
        )
        .attach(Template::fairing())
}
