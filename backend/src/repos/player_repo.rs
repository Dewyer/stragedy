use mongodb::sync::{Collection};
use crate::models::player::Player;
use std::error::Error;
use bson;

pub struct PlayerRepo
{
	player_collection: Collection,
}

impl PlayerRepo {
	pub fn new(player_collection: Collection) -> Self
	{
		PlayerRepo { player_collection }
	}

	pub fn insert_player(&self,player:&Player) -> Result<(),Box<dyn Error>>
	{
		let doc =  bson::to_bson(player)?;
		let doc = doc.as_document().ok_or(std::io::Error::new(std::io::ErrorKind::Other,"Failed docking!"))?;
		self.player_collection.insert_one(doc.clone(),None)?;
		Ok(())
	}

	pub fn find_player_by_username(&self,username:&str) -> Option<Player>
	{
		let querry_res:Result<Option<bson::Document>,mongodb::error::Error> = self.player_collection.find_one(doc!{"username":username},None);

		match querry_res
		{
			Ok(doc) => match doc
			{
				Some(doc_val) => {
					match bson::from_bson::<Player>(Bson::from(doc_val)) {
						Ok(val)=> Some(val),
						Err(_)=>None
					}
				},
				None=> None
			},
			Err(_)=> None,
		}
	}
}
