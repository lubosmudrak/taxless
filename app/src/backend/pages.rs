use rocket_dyn_templates::{Template, context};
use rocket::http::CookieJar;

///Main webpage of the project
#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Template {
    // Toto vsetko len placeholder hej!

    let login_state: bool = match cookies.get("login_state"){
        Some(login_state) => {
            if login_state.value() == "true"{
                true
            } else {
                false
            }
        },
        None => false
    };
    let username = match cookies.get("username"){
        Some(username) => username.value().to_string(),
        None => "USERNAME_ERROR".to_string()
    };
    Template::render("index", context! {
        page_title: "TAXLESS",
        user_logged_in: login_state,
        user_name: username
    })
}

/// The front page for request for remittance of two percent of income tax
#[get("/dve_percenta")]
pub fn dve_percenta() -> Template{
    Template::render("dve_percenta", context!{
        page_title: "Poukázanie 2% dane"
    })
}

/// Screen for login and user registration
#[get("/prihlasenie")]
pub fn prihlasenie() -> Template{
    Template::render("Prihlasenie", context!{
        page_title: "Prihlásenie"
    })
}

/// User panel for the logged in user
#[get("/pouzivatelsky_panel")]
pub fn pouzivatelsky_panel() -> Template{
    Template::render("pouzivatelsky_panel", context!{
        page_title: "Používateľsky panel"
    })
}

/// The front page for application for income tax settlement
#[get("/zuctovanie_ziadost")]
pub fn zuctovanie_ziadost() -> Template{
    Template::render("zuctovanie_ziadost", context!{
        page_title: "Zúčtovanie"
    })
}



//UNIT TESTS
#[cfg(test)]
mod pages_tests{
    use super::*;
}
