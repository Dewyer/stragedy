#![feature(proc_macro_hygiene, decl_macro)]
mod controllers;

#[macro_use] extern crate rocket;
extern crate lib;

//const MONGO_URL:String = "mongodb+srv://roottwo:5866I9FZlTsbApoX@cluster0-m3otq.mongodb.net/stragedy?retryWrites=true&w=majority".to_string();

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/api", routes![index]).launch();
}