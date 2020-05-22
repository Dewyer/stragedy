#![feature(proc_macro_hygiene, decl_macro)]
pub mod repos;
pub mod controllers;
pub mod services;
pub mod models;
pub mod helpers;

#[cfg(test)] pub mod tests;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate dotenv_codegen;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mongodb;
#[macro_use] extern crate bson;
#[macro_use] extern crate lazy_static;

extern crate lib;

use crate::repos::Repo;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket::http::Method;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

pub fn rocket(seed:bool) -> rocket::Rocket
{
	println!("Galaxy config {:?}",services::config::get_config());
	println!("Spinning up database!");
	let repo = Repo::new();
	if repo.is_err()
	{
		println!("db error, cannot start");
		panic!();
	}

	let repo = repo.unwrap();
	println!("Seeding database!");
	if (seed) {
		repos::seeder::seed(&repo);
	}

	let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:5000"]);
	let cors = rocket_cors::CorsOptions {
		allowed_origins,
		allowed_methods: vec![Method::Get,Method::Post,Method::Put].into_iter().map(From::from).collect(),
		allowed_headers: AllowedHeaders::All,
		allow_credentials: true,
		..Default::default()
	}.to_cors().unwrap();

	rocket::ignite().mount("/api", routes![index,
        crate::controllers::public_sites::register,
		crate::controllers::public_sites::login,
		crate::controllers::public_sites::who_am_i
    ])
	.manage(repo)
	.attach(cors)
}
