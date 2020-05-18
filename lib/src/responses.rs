
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPlayerResponse
{
	pub jwt:String,
	pub expiration_tick:i64
}
