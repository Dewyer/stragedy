use crate::models::resource::GameRes;
use chrono::prelude::*;

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
}
