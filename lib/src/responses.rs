
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginPlayerResponse
{
	pub jwt:String,
	pub expiration_tick:i64
}
