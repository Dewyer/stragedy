#![feature(proc_macro_hygiene, decl_macro)]
pub mod repos;
pub mod controllers;
pub mod services;
pub mod models;
pub mod error;
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

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

pub fn rocket() -> rocket::Rocket
{
	println!("Spinning up database!");
	let repo = Repo::new();
	if repo.is_err()
	{
		println!("db error, cannot start");
		panic!();
	}

	let repo = repo.unwrap();
	println!("Seeding database!");
	repos::seeder::seed(&repo);

	rocket::ignite().mount("/api", routes![index,
        crate::controllers::public_sites::register,
        crate::controllers::public_sites::login
    ])
	.manage(repo)
}
