#![feature(proc_macro_hygiene, decl_macro)]
pub mod repos;
pub mod controllers;
pub mod services;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate dotenv_codegen;
extern crate lib;

use crate::repos::Repo;

//const MONGO_URL:String = "mongodb+srv://roottwo:5866I9FZlTsbApoX@cluster0-m3otq.mongodb.net/stragedy?retryWrites=true&w=majority".to_string();

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main()
{
	println!("Spinning up database!");
	let repo = Repo::new();
	if repo.is_err()
	{
		println!("db error, cannot start");
		panic!();
	}

	let repo = repo.unwrap();
    rocket::ignite().mount("/api", routes![index,
        crate::controllers::public_sites::register
    ])
	.manage(repo)
    .launch();
}
