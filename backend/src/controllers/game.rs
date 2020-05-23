use rocket_contrib::json::Json;
use lib::{ApiResponse};
use crate::services;
use crate::repos::Repo;
use lib::requests;
use lib::responses;
use rocket::State;
use lib::models::player::Player;
use lib::error::{AuthError, ApiError};
use crate::models::player::PlayerGuard;
use lib::config::GalaxyConfig;

#[get("/base")]
pub fn base_info(auth:PlayerGuard,repo:State<Repo>) -> Json<ApiResponse<responses::BaseResponse,ApiError>>
{
	let base_data_res =  services::game_manager::get_base_data(&auth.player, repo.inner());
	Json(ApiResponse::from_result(base_data_res))
}

#[get("/world_info")]
pub fn galaxy_info() ->Json<ApiResponse<GalaxyConfig,ApiError>>
{
	Json(ApiResponse{
		content:Some(services::config::get_config().clone()),
		error: ApiError::NoError
	})
}
