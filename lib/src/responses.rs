use crate::models::player::PlayerDto;
use crate::models::planet::Planet;
use crate::config::GalaxyConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginPlayerResponse
{
	pub jwt:String,
	pub expiration_tick:i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseResponse
{
	pub player: PlayerDto,
	pub base_planet:Planet,
	pub config: GalaxyConfig,
}

