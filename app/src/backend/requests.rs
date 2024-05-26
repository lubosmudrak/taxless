use super::pages;

use rocket::form::Form;
use rocket::http::{CookieJar, Cookie};
use rocket::response::Redirect;


#[derive(FromForm)]
pub struct Credentials {
    username: String,
    password: String,
}
//TODO: implement login. This is currently a placeholder to test UI for case of user login
#[post("/prihlasovacie_udaje", data = "<credentials>")]
pub fn login_request(cookies: &CookieJar<'_>, credentials: Form<Credentials>) -> Redirect{
    cookies.add(Cookie::new("login_state", "true"));
    cookies.add(Cookie::new("username", credentials.username.to_string()));

    Redirect::to(uri!(pages::pouzivatelsky_panel))
}

#[post("/logout")]
pub fn logout_request(cookies: &CookieJar<'_>)-> Redirect{
    cookies.remove_private("username");
    cookies.add(Cookie::new("login_state", "false"));

    Redirect::to(uri!(pages::index))
}