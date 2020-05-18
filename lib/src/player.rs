#[derive(Serialize, Deserialize, Debug)]
pub struct Player
{
	pub username:String,
	pub email:String,
	pub password_salt:String,
	pub password_hash:String
}
