extern crate rocket;
// use bson::Document;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
pub mod models;
mod services;

// use handlebars::{
//     Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output, RenderContext,
//     RenderError,
// };
// use std::io::Write;

// fn unique(
//     h: &Helper,
//     _: &Handlebars,
//     _: &Context,
//     rc: &mut RenderContext,
//     out: &mut dyn Output,
// ) -> HelperResult {
//     let param = h.param(0).unwrap();
//     println!("{:?}", param);
//     out.write(param.value().render().as_ref())?;
//     Ok(())
// }

#[launch]
fn rocket() -> _ {
    // let mut handlebars = Handlebars::new();

    // handlebars.register_helper("unique", Box::new(unique));

    rocket::build()
        .mount(
            "/",
            routes![
                services::simulate,
                services::index,
                services::indexx,
                services::file_json,
                // services::store_p,
                // services::store_t,
                services::test,
                services::stream
            ],
        )
        .attach(Template::fairing())
}
