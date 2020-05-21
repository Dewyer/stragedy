use lib::error::ApiError;
use lib::responses::LoginPlayerResponse;
use yew::services::StorageService;
use yew::services::storage::Area;

pub fn save_auth_credentials(jwt_resp : LoginPlayerResponse) -> ApiError
{
	let mut storage_res = StorageService::new(Area::Local);
	if let Ok(storage) = storage_res.as_mut()
	{
		storage.store("jwt",Ok(jwt_resp.jwt));
		storage.store("jwt-exp",Ok(jwt_resp.expiration_tick.to_string()));
		ApiError::NoError
	}
	else
	{
		ApiError::Other("No local storage".to_string())
	}
}
