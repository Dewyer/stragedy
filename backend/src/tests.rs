use rocket::local::Client;
use rocket::http::{ContentType, Cookie,Status};
use lib::requests;
use lib::responses;

#[test]
pub fn register()
{
	let client = Client::new(super::rocket()).expect("valid rocket instance");
	let mut response = client.post("/api/register")
		.header(ContentType::JSON)
		.body(serde_json::to_string(&requests::CreatePlayerRequest{
			username: "bela".to_string(),
			email: "bela@gmail.com".to_string(),
			password: "Test12345".to_string()
		}).unwrap())
		.dispatch();

	assert_eq!(response.status(),Status::Ok);
	let resp_obj = serde_json::from_str::<lib::ApiEmptyResponse>(&response.body_string().unwrap()).unwrap();
	println!("{:?}",resp_obj);
	assert_eq!(resp_obj.error,false);
}
