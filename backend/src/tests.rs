use rocket::local::Client;
use rocket::http::{ContentType, Cookie,Status,Header};
use lib::requests;
use lib::responses;
use crate::services;
use lib::error::AuthError;

#[test]
pub fn register()
{
	let client = Client::new(super::rocket(false)).expect("valid rocket instance");
	let mut response = client.post("/api/register")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::CreatePlayerRequest{
			username: format!("bela{}",services::crypto::generate_random_crypto_string(5)),
			email: format!("bela{}@gmail.com",services::crypto::generate_random_crypto_string(5)),
			password: "Test12345".to_string()
		}).unwrap())
		.dispatch();

	assert_eq!(response.status(),Status::Ok);
	let resp_obj = serde_json::from_str::<lib::ApiResponse<(),AuthError>>(&response.body_string().unwrap()).unwrap();
	println!("{:?}",resp_obj);
	assert_eq!(resp_obj.error,AuthError::NoError);
}

#[test]
pub fn login_and_use_jwt()
{
	let client = Client::new(super::rocket(true)).expect("valid rocket instance");
	let mut response = client.post("/api/login")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::LoginPlayerRequest{
			username:"test1".to_string(),
			password:"Test12345".to_string()
		}).unwrap())
		.dispatch();
	assert_eq!(response.status(),Status::Ok);
	let resp_obj = serde_json::from_str::<lib::ApiResponse<responses::LoginPlayerResponse,AuthError>>(&response.body_string().unwrap()).unwrap();
	println!("{:?}",resp_obj);
	assert_eq!(resp_obj.error,AuthError::NoError);

	// Npw lets use that for authorization
	let jwt = resp_obj.content.unwrap().jwt.clone();
	println!("trying jwt : {}",jwt);
	let mut resp2 = client.get("/api/who")
		.header(Header::new("Authorization", jwt))
		.dispatch();

	assert_eq!(resp2.status(),Status::Ok);
	let resp2_obj = serde_json::from_str::<lib::ApiResponse<(),AuthError>>(&resp2.body_string().unwrap()).unwrap();
	assert_eq!(resp2_obj.error,AuthError::Other("test1".to_string()));
}
