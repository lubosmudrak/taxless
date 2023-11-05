#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

#[path ="backend/pages.rs"]
mod pages;

/// Function for launching the web app and mounting the application's file storage
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![pages::index])
    .mount("/", FileServer::from(relative!("src/frontend")))
    .attach(Template::fairing())
}
