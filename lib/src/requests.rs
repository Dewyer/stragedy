
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlayerRequest
{
	username:String,
	email:String,
	password:String
}
