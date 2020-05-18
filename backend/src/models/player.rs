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
	pub sec_token:String,
	pub expiration:DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
	pub player_id: String,
	pub token: String
}
