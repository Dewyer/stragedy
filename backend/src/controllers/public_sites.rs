use rocket_contrib::json::Json;
use lib::{ApiResponse};
use crate::services;
use crate::repos::Repo;
use lib::requests;
use lib::responses;
use rocket::State;
use crate::models::player::Player;
use lib::error::AuthError;

#[post("/register",data="<player>")]
pub fn register(player:Json<requests::CreatePlayerRequest>,repo:State<Repo>) -> Json<ApiResponse<(),AuthError>>
{
	let res =services::auth::create_player(player.into_inner(),&repo);
	Json(ApiResponse
	{
		content:None,
		error:match res {Ok(v)=>AuthError::NoError,Err(e)=>e},
	})
}

#[post("/login",data="<login_info>")]
pub fn login(login_info:Json<requests::LoginPlayerRequest>, repo:State<Repo>) -> Json<ApiResponse<responses::LoginPlayerResponse,AuthError>>
{
	let resp = services::auth::login_player(login_info.into_inner(),&repo);
	Json(ApiResponse
	{
		content:match resp.as_ref() {Ok(val)=> Some(val.to_owned()), Err(_) => None },
		error: match resp.as_ref() {Ok(v)=>AuthError::NoError,Err(err)=>err.clone()},
	})
}

#[get("/who")]
pub fn who_am_i(player:Player) -> Json<ApiResponse<(),AuthError>>
{
	Json(ApiResponse{
		content:None,
		error:AuthError::Other(player.username.clone()),
	})
}
