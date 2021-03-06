use rocket::local::{LocalResponse,LocalRequest,Client};
use rocket::http::{ContentType, Cookie,Status,Header};
use lib::{requests, ApiResponse};
use lib::responses;
use crate::services;
use lib::error::{AuthError, ApiError};

pub fn login_get_jwt(client:&Client,username:&str) -> String
{
	let mut response = client.post("/api/login")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::LoginPlayerRequest{
			username:username.to_string(),
			password:"Test12345".to_string()
		}).unwrap())
		.dispatch();
	get_jwt_from_response(&mut response)
}

pub fn get_jwt_from_response(resp:&mut LocalResponse) -> String
{
	let bdy_str = resp.body_string().as_ref().unwrap().clone();
	let out = serde_json::from_str::<lib::ApiResponse<responses::LoginPlayerResponse,AuthError>>(&bdy_str).expect("Couldn't deserialize jwt resp.");
	if out.error != AuthError::NoError
	{
		println!("Login will fail : {:?}",out.error);
	}
	out.content.as_ref().unwrap().jwt.clone()
}

pub fn get_client(seeded:bool) -> Client
{
	Client::new(super::rocket(seeded)).expect("valid rocket instance")
}

pub fn register_user<'c>(client:&'c Client,username:&str,email:&str) -> LocalResponse<'c>
{
	client.post("/api/register")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::CreatePlayerRequest{
			username: username.to_string(),
			email: email.to_string(),
			password: "Test12345".to_string()
		}).unwrap())
	.dispatch()
}

#[test]
pub fn register()
{
	let client = get_client(false);
	let mut response = register_user(&client,
	&format!("bela{}",services::crypto::generate_random_crypto_string(5)),
	 &format!("bela{}@gmail.com",services::crypto::generate_random_crypto_string(5)));

	assert_eq!(response.status(),Status::Ok);
	let resp_obj = serde_json::from_str::<lib::ApiResponse<(),AuthError>>(&response.body_string().unwrap()).unwrap();
	println!("{:?}",resp_obj);
	assert_eq!(resp_obj.error,AuthError::NoError);
}

#[test]
pub fn login_and_use_jwt()
{
	let client = get_client(true);
	register_user(&client,"test1","test1@gmail.com");
	// Npw lets use that for authorization
	let jwt = login_get_jwt(&client,"test1");
	println!("trying jwt : {}",jwt);
	let mut resp2 = client.get("/api/who")
		.header(Header::new("Authorization", jwt))
		.dispatch();

	assert_eq!(resp2.status(),Status::Ok);
	let resp2_obj = serde_json::from_str::<lib::ApiResponse<(),AuthError>>(&resp2.body_string().unwrap()).unwrap();
	assert_eq!(resp2_obj.error,AuthError::Other("test1".to_string()));
}

#[test]
pub fn base_info()
{
	let client = get_client(false);
	let mut reg_resp = register_user(&client,"test2","test2@gmail.com");
	let reg_obj = serde_json::from_str::<ApiResponse<(),AuthError>>(&reg_resp.body_string().as_ref().unwrap().clone()).unwrap();
	println!("Base reg resp: {:?}",reg_obj);
	let jwt = login_get_jwt(&client,"test2");

	let mut response = client.get("/api/base").header(Header::new("Authorization",jwt)).dispatch();
	assert_eq!(response.status(),Status::Ok);
	let r_obj = serde_json::from_str::<ApiResponse<responses::BaseResponse,ApiError>>(&response.body_string().as_ref().unwrap().clone()).unwrap();
	assert_eq!(r_obj.error,ApiError::NoError);

	assert!(r_obj.content.is_some());
	assert!(r_obj.content.unwrap().player.username == "test2");
}
