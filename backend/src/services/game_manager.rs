use lib::responses::BaseResponse;
use crate::repos::Repo;
use lib::models::player::{Player, PlayerDto};
use crate::services;
use lib::error::ApiError;

pub fn get_base_data(player:&Player,repo:&Repo) -> Result<BaseResponse,ApiError>
{
	let mut planets = services::planet_manager::get_all_planets_of_player(player.get_cpy_id(),repo)?;
	let p_dto = PlayerDto::from_player_and_planets(player,planets.iter().collect());

	Ok(BaseResponse
	{
		player:p_dto,
		base_planet:planets.remove(0)
	})
}
