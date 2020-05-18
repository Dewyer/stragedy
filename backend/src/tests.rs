use rocket::local::Client;
use rocket::http::{ContentType, Cookie,Status};
use lib::requests;
use lib::responses;
use crate::services;

#[test]
pub fn register()
{
	let client = Client::new(super::rocket()).expect("valid rocket instance");
	let mut response = client.post("/api/register")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::CreatePlayerRequest{
			username: format!("bela{}",services::crypto::generate_random_crypto_string(5)),
			email: format!("bela{}@gmail.com",services::crypto::generate_random_crypto_string(5)),
			password: "Test12345".to_string()
		}).unwrap())
		.dispatch();

	assert_eq!(response.status(),Status::Ok);
	let resp_obj = serde_json::from_str::<lib::ApiEmptyResponse>(&response.body_string().unwrap()).unwrap();
	println!("{:?}",resp_obj);
	assert_eq!(resp_obj.error,false);
}

#[test]
pub fn login_and_use_jwt()
{
	let client = Client::new(super::rocket()).expect("valid rocket instance");
	let mut response = client.post("/api/login")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::LoginPlayerRequest{
			username:"test1".to_string(),
			password:"Test12345".to_string()
		}).unwrap())
		.dispatch();
	assert_eq!(response.status(),Status::Ok);
	let resp_obj = serde_json::from_str::<lib::ApiResponse<responses::LoginPlayerResponse>>(&response.body_string().unwrap()).unwrap();
	println!("{:?}",resp_obj);
	assert_eq!(resp_obj.error,false);
}
