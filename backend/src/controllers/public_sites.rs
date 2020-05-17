use rocket_contrib::json::Json;
use lib::ApiEmptyResponse;

#[get("/register")]
pub fn register() -> Json<ApiEmptyResponse>
{
	Json(ApiEmptyResponse
	{
		error:false,
		status:"hello".to_string()
	})
}