use rocket_contrib::json::Json;
use lib::ApiEmptyResponse;
use crate::services;
use crate::repos::Repo;
use lib::requests::CreatePlayerRequest;
use rocket::State;

#[get("/register",data="<player>")]
pub fn register(player:Json<CreatePlayerRequest>,repo:State<Repo>) -> Json<ApiEmptyResponse>
{
	let res =services::auth::create_player(player.into_inner(),&repo);

	Json(ApiEmptyResponse
	{
		error:res.is_err(),
		status:"".to_string()
	})
}
