#[derive(Serialize, Deserialize, Debug)]
pub struct Player
{
	username:String,
	email:String,
	password_salt:String,
	password_hash:String
}
