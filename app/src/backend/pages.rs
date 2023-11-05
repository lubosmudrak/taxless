use rocket_dyn_templates::{Template, context};

///Main webpage of the project
#[get("/")]
pub fn index() -> Template{
    Template::render("index", context!{})
}

//UNIT TESTS
#[cfg(test)]
mod pages_tests{
    use super::*;
}
