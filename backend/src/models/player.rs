use bson;
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

	pub token:Option<PlayerToken>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerToken
{
	sec_token:String,
	expiration:DateTime<Utc>
}


