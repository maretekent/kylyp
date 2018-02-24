#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![recursion_limit = "128"]
#![feature(decl_macro)]
#![feature(attr_literals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate dotenv;
extern crate chrono;
//extern crate easy;
extern crate regex;
extern crate config;

#[macro_use]
mod controller;
#[macro_use]
mod handler;
#[macro_use]
mod model;
mod utils;

use rocket_contrib::Template;

use controller::{home,user,article};

const CFG_DEFAULT: &'static str = "Rocket";

fn main() {
    rocket::ignite()
        .mount("/", routes![home::public,home::index_user,home::index,home::doc_user,home::doc])
        .mount("/user",routes![user::login_register,user::register,user::register_post,
                               user::login_user,user::login,user::login_post,user::user_page_login,user::logout])
        .mount("/article",routes![article::article,article::add_comment,article::article_nouser,article::new,article::add_article])
        .attach(Template::fairing())
        .launch();
}



