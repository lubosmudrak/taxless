use super::pages;

use rocket::form::Form;
use rocket::http::{CookieJar, Cookie};
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};



//TODO: password cannot be in a public struct. This is a serious security risk.
#[derive(FromForm)]
pub struct Credentials {
    username: String,
    password: String,
}

enum PersonalInfoError {
    FirstNameContainsNumbers,
    FirstNameMissing,
    FirstNameTooLong,

    LastNameMissing,
    LastNameTooLong,

    TitleAfterContainsNumbers,
    TitleAfterTooLong,

    TitleBeforeContainsNumbers,
    TitleBeforeTooLong,

    IdentificationNumberMissing,
    InvalidIDFormat,

    BuildingNumberMissing,
    BuildingNumberTooLong,

    CityMissing,
    CityTooLong,

    CountryMissing,
    CountryTooLong,

    PostalCodeContainsLetters,
    PostalCodeInvalidLength,
    PostalCodeMissing,

    StreenNameTooLong,

    EmployerMissing,
    EmployerTooLong,

    MissingRequiredFields,
    StringTooLong,
    UnknownError,
    None
}

#[derive(FromForm)]
pub struct PersonalInfo{
    /// Krstné meno žiadateľa
    first_name: Option<String>,
    /// Priezvisko žiadateľa
    last_name: Option<String>,
    /// Titul za menom. Žiadateľ bez titulu za menom tento údaj nevypisuje
    title_after: Option<String>,
    /// Titul pred menom. Žiadateľ bez titulu za menom tento údaj nevypisuje
    title_before: Option<String>,
    /// Štátom pridelené rodné číslo pridelené štátom
    identification_number: Option<String>,

    /// Súpisné alebo orientačné číslo domu / bytu žiadateľa. Môže obsahovať lomítka a písmená
    building_number: Option<String>,
    /// Mesto alebo obec trvalého bydliska žiadateľa
    city: Option<String>,
    /// Krajina trvalého pobytu žiadateľa
    country: Option<String>,
    ///Poštové smerové číslo obce alebo mesta žiadateľa. Obsahuje iba číslice, môže začínať nulou a musí mať 5 číslic
    postal_code: Option<String>,
    ///Ulica bydliska žiadateľa
    street: Option<String>,

    ///Názov a adresa zamestnávateľa žiadateľa
    employer: Option<String>,

    ///Je žiadateľ daňovým rezidentom?
    is_tax_resident: Option<bool>,
    ///Je žiadateľ dôchodca?
    is_pensioner: Option<bool>,
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


#[post("/osobne_info", data = "<personal_info>")]
pub fn post_personal_info(cookies: &CookieJar<'_>, personal_info: Form<PersonalInfo>) -> Template{

    let mut validation_errors: Vec<PersonalInfoError> = Vec::new();

    match &personal_info.first_name {
        Some(first_name) => {

            match first_name.len() {
                0 => { validation_errors.push(PersonalInfoError::FirstNameMissing) },
                1..=100 => {
                    if first_name.to_string().chars().all(char::is_alphabetic) {
                        cookies.add(Cookie::new("first_name", first_name.to_string()));
                    } else {validation_errors.push(PersonalInfoError::FirstNameContainsNumbers)}
                },
                _ => {validation_errors.push(PersonalInfoError::FirstNameTooLong)}
            }
        },
        None => { validation_errors.push(PersonalInfoError::FirstNameMissing) }
    } 

    match &personal_info.last_name {
        Some(last_name) => {

            match last_name.len() {
                0 => { validation_errors.push(PersonalInfoError::LastNameMissing) },
                1..=100 => { cookies.add(Cookie::new("last_name", last_name.to_string())); },
                _ => { validation_errors.push(PersonalInfoError::LastNameTooLong) }
            }
        },
        None => {
            validation_errors.push(PersonalInfoError::LastNameMissing)
        }
    }

    let mut title_after_length_ok: bool = false;
    let mut title_after_format_ok: bool = false;
    match &personal_info.title_after {

        Some(title_after) => {
            if title_after.len() <=20 { title_after_length_ok = true;
            } else { validation_errors.push(PersonalInfoError::TitleAfterTooLong) }

            if title_after.chars().all(char::is_alphabetic) == true { title_after_format_ok = true;
            } else { validation_errors.push(PersonalInfoError::TitleAfterContainsNumbers) }

            if title_after_format_ok == true && title_after_length_ok == true {
                cookies.add(Cookie::new("title_after", title_after.to_string()));
            }
        },
        None => {
            cookies.add(Cookie::new("title_after", ""));
        }
    }

    let mut title_before_length_ok: bool = false;
    let mut title_before_format_ok: bool = false;
    match &personal_info.title_before {
        Some(title_before) => {

            if title_before.len() <= 20{ title_before_length_ok = true;
            } else { validation_errors.push(PersonalInfoError::TitleBeforeContainsNumbers) }

            if title_before.to_string().chars().all(char::is_alphabetic) == true { title_before_format_ok = true;
            } else { validation_errors.push(PersonalInfoError::TitleBeforeContainsNumbers) }

            if title_before_length_ok == true && title_before_format_ok == true {
                cookies.add(Cookie::new("title_before", title_before.to_string()));
            }
        },
        None => {
            cookies.add(Cookie::new("title_before", ""));
        }
    }

    match &personal_info.identification_number {
        Some(personal_id) => {
            match &personal_id.chars().nth(8) {
                Some(char) => {
                    if *char == '/' {
                        match &personal_id.len() {
                            9..=10 => { cookies.add(Cookie::new("identification_number",personal_id.to_string())); }
                            _ => { validation_errors.push(PersonalInfoError::InvalidIDFormat) }
                        }
                    }
                },
                None => {validation_errors.push(PersonalInfoError::InvalidIDFormat)}
            }
        },
        None => {
            validation_errors.push(PersonalInfoError::IdentificationNumberMissing);
        }
    }

    match &personal_info.building_number {
        Some(building_number) => {
            if building_number.len() <= 10 {
                cookies.add(Cookie::new("building_number",building_number.to_string()))
            } else { validation_errors.push(PersonalInfoError::BuildingNumberTooLong) }
        },
        None => {validation_errors.push(PersonalInfoError::BuildingNumberMissing)}
    }

    match &personal_info.city {
        Some(city) => { 
            if city.len() <= 100 {
                cookies.add(Cookie::new("city",city.to_string()))
            } else { validation_errors.push(PersonalInfoError::CityTooLong) }
        },
        None => {validation_errors.push(PersonalInfoError::CityMissing)}
    }

    match &personal_info.country {
        Some(country) => {
            if country.len() <= 40 {
                cookies.add(Cookie::new("country",country.to_string()))
            } else { validation_errors.push(PersonalInfoError::CountryTooLong) }
        },
        None => {validation_errors.push(PersonalInfoError::CountryMissing)}
    }

    let mut postal_code_length_ok: bool = false;
    let mut postal_code_format_ok: bool = false;

    match &personal_info.postal_code {
        Some(postal_code) => {

            if postal_code.len() == 5 { postal_code_length_ok = true
            } else { validation_errors.push(PersonalInfoError::PostalCodeInvalidLength) }

            if postal_code.chars().all(char::is_numeric) == true { postal_code_format_ok = true
            } else { validation_errors.push(PersonalInfoError::PostalCodeContainsLetters) }

            if postal_code_length_ok == true && postal_code_format_ok == true {
                cookies.add(Cookie::new("postal_code",postal_code.to_string()));
            }
        },
        None => {validation_errors.push(PersonalInfoError::PostalCodeMissing)}
    }

    match &personal_info.street {
        Some(street) => {
            if street.len() <= 100 {
                cookies.add(Cookie::new("street", street.to_string()));
            } else { validation_errors.push(PersonalInfoError::StreenNameTooLong) }
        },
        None => {
            cookies.add(Cookie::new("street", ""));
        }
    }

    match &personal_info.employer {
        Some(employer) => {
            if employer.len() <= 100 { cookies.add(Cookie::new("employer", employer.to_string()))
            } else { validation_errors.push(PersonalInfoError::EmployerTooLong) }
        },
        None => { validation_errors.push(PersonalInfoError::EmployerMissing) }
    }

    match &personal_info.is_tax_resident {
        Some(is_tax_resident) => { cookies.add(Cookie::new("is_tax_resident", is_tax_resident.to_string())) },
        None => { cookies.add(Cookie::new("is_tax_resident", false.to_string())) }
    }
    cookies.add(Cookie::new("is_pensioner",personal_info.is_tax_resident.unwrap().to_string()));

    if validation_errors.len() == 0 {
        pages::zuctovanie_ziadost_nezdanitelne_casti()
    } else {
        pages::zuctovanie_ziadost()
    }

}