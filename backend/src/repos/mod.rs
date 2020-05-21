pub mod generic_repo;
pub mod seeder;

use mongodb::sync::{Client,Database};
use bson::Document;
use std::error::Error;
use lib::models::player::Player;
use crate::repos::generic_repo::GenericRepo;
use lib::models::planet::Planet;

pub struct Repo
{
    client:Client,
	database:Database,

	pub player_repo:GenericRepo<Player>,
	pub planet_repo:GenericRepo<Planet>
}

impl Repo
{
    pub fn new() -> Result<Self,Box<dyn Error>>
    {
        let constr = dotenv!("MONGO_CON");
        let client = Client::with_uri_str(constr)?;
        let db = client.database("stragedy");
		let pc = db.collection("players");
		let pl = db.collection("planets");

        Ok(Repo
        {
            client:client,
			database:db,
			player_repo: GenericRepo::new(pc),
			planet_repo: GenericRepo::new(pl)
        })
    }
}
