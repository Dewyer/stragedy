
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlayerRequest
{
	pub username:String,
	pub email:String,
	pub password:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPlayerRequest
{
	pub username:String,
	pub password:String
}
