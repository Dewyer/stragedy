use rocket_contrib::json::Json;
use lib::{ApiEmptyResponse,ApiResponse};
use crate::services;
use crate::repos::Repo;
use lib::requests;
use lib::responses;
use rocket::State;
use std::error::Error;
use lib::responses::LoginPlayerResponse;

#[post("/register",data="<player>")]
pub fn register(player:Json<requests::CreatePlayerRequest>,repo:State<Repo>) -> Json<ApiEmptyResponse>
{
	let res =services::auth::create_player(player.into_inner(),&repo);
	Json(ApiEmptyResponse
	{
		error:res.is_err(),
		status:match res { Ok(_) => "".to_string() , Err(ee)=>ee.get_status().to_string()}
	})
}

#[post("/login",data="<login_info>")]
pub fn login(login_info:Json<requests::LoginPlayerRequest>, repo:State<Repo>) -> Json<ApiResponse<responses::LoginPlayerResponse>>
{
	let resp = services::auth::login_player(login_info.into_inner(),&repo);
	Json(ApiResponse
	{
		content:match resp.as_ref() {Ok(val)=> Some(val.to_owned()), Err(_) => None },
		error:resp.as_ref().is_err(),
		status:match resp.as_ref() {Ok(_) => "".to_string(),Err(err)=> err.get_status().to_string()}
	})
}
