
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlayerRequest
{
	pub username:String,
	pub email:String,
	pub password:String
}
