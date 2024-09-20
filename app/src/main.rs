#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

#[path ="backend/pages.rs"]
mod pages;
#[path = "backend/requests.rs"]
mod requests;

/// Function for launching the web app and mounting the application's file storage
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![
        pages::index,
        pages::dve_percenta,
        pages::prihlasenie,
        pages::pouzivatelsky_panel,
        pages::zuctovanie_ziadost,
        pages::zuctovanie_ziadost_nezdanitelne_casti,
        requests::login_request,
        requests::logout_request,
        requests::post_personal_info
    ])
    .mount("/js", FileServer::from("src/frontend/js"))
    .mount("/img", FileServer::from("src/frontend/img"))
    .mount("/css", FileServer::from("src/frontend/css"))
    .attach(Template::fairing())

}
