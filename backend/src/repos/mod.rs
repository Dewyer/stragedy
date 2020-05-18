pub mod player_repo;

use mongodb::sync::{Client,Database};
use bson::Document;
use std::error::Error;
use crate::repos::player_repo::PlayerRepo;

pub struct Repo
{
    client:Client,
	database:Database,

	pub player_repo:PlayerRepo
}

impl Repo
{
    pub fn new() -> Result<Self,Box<dyn Error>>
    {
        let constr = dotenv!("MONGO_CON");
        let client = Client::with_uri_str(constr)?;
        let db = client.database("stragedy");
		let pc = db.collection("players");

        Ok(Repo
        {
            client:client,
			database:db,
			player_repo: PlayerRepo::new(pc)
        })
    }
}

/*
pub fn easy_to_doc<'r,T>(modell:T) -> &'r Document
{

}
*/
