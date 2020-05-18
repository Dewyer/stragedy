use rocket_contrib::json::Json;
use lib::{ApiEmptyResponse,ApiResponse};
use crate::services;
use crate::repos::Repo;
use lib::requests;
use lib::responses;
use rocket::State;
use std::error::Error;
use lib::responses::LoginPlayerResponse;

#[get("/register",data="<player>")]
pub fn register(player:Json<requests::CreatePlayerRequest>,repo:State<Repo>) -> Json<ApiEmptyResponse>
{
	let res =services::auth::create_player(player.into_inner(),&repo);
	Json(ApiEmptyResponse
	{
		error:res.is_err(),
		status:match res { Ok(_) => "".to_string() , Err(ee)=> ee.description().to_string()}
	})
}

#[get("/login",data="<login_info>")]
pub fn login(login_info:Json<requests::LoginPlayerRequest>, repo:State<Repo>) -> Json<ApiResponse<responses::LoginPlayerResponse>>
{
	Json(ApiResponse
	{
		content:Some(LoginPlayerResponse
		{
			jwt:"".to_string(),
			expiration_tick:0
		}),
		error:false,
		status:"".to_string()
	})
}
