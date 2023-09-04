#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[get("/about")]
fn about() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("about", &context)
}

#[get("/projects")]
fn projects() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("projects", &context)
}

#[get("/contact")]
fn contact() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("contact", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, about, projects, contact])
        .attach(Template::fairing())
        .launch();
}