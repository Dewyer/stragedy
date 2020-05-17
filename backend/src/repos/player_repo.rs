use mongodb::sync::{Collection,Document};
use lib::player::Player;
use std::error::Error;

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
		let doc =  bson::to_bson(player)?.as_document().ok_or(std::io::Error::new(std::io::ErrorKind::Other,"Failed docking!"))?;
		self.player_collection.insert_one(doc.clone(),None);
		Ok(())
	}
}
