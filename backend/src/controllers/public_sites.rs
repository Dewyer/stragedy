use rocket_contrib::json::Json;
use lib::ApiEmptyResponse;
use crate::services;
use crate::repos::Repo;

#[get("/register")]
pub fn register(repo:Repo) -> Json<ApiEmptyResponse>
{
	println!("not an error");
	services::auth::create_user(&repo);
	
	Json(ApiEmptyResponse
	{
		error:false,
		status:"hello".to_string()
	})
}