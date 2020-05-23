use crate::models::resource::GameRes;
use chrono::prelude::*;
use crate::models::planet::{PlanetInfoDto, Planet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player
{
	#[serde(rename = "_id")]
	pub id: Option<bson::oid::ObjectId>,
	pub username:String,
	pub email:String,
	pub password_salt:String,
	pub password_hash:String,

	pub token:Option<PlayerToken>,

	//Gameplay
	pub galactic_credits:GameRes,
	pub points:i32,
	pub controlled_planet_ids: Vec<bson::oid::ObjectId>
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct PlayerDto
{
	pub username:String,
	pub galactic_credits:GameRes,
	pub points:i32,
	pub controlled_planets: Vec<PlanetInfoDto>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerToken
{
	pub sec_token:String,
	pub expiration:DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
	pub player_id: String,
	pub token: String,
	pub exp: u64
}

#[derive(Debug)]
pub enum AuthFail
{
	NoToken,
	InvalidToken,
	Expired,
	DbError
}

impl Player
{
	pub fn new(username:&str,email:&str,password_hash:&str,password_salt:&str) -> Self
	{
		Player{
			id:Some(bson::oid::ObjectId::new().unwrap()),
			username:username.to_string(),
			email:email.to_string(),
			password_hash:password_hash.to_string(),
			password_salt:password_salt.to_string(),
			token:None,

			galactic_credits:GameRes::new(0),
			points:0,
			controlled_planet_ids: vec![]
		}
	}

	pub fn get_cpy_id(&self) -> bson::oid::ObjectId
	{
		self.id.as_ref().unwrap().clone()
	}

	pub fn get_id(&self) ->&bson::oid::ObjectId
	{
		self.id.as_ref().unwrap()
	}
}

impl PlayerDto
{
	pub fn from_player_and_planets(player:&Player,planets:Vec<&Planet>) -> Self
	{
		Self
		{
			username:player.username.clone(),
			galactic_credits: player.galactic_credits,
			points: player.points,
			controlled_planets: planets.iter().map(|elem| PlanetInfoDto::from_planet(elem)).collect()
		}
	}
}
