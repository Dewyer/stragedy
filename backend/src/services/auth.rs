use crate::repos::Repo;
use crate::models::player::Player;
use lib::requests;
use lib::responses;
use std::error::Error;
use crate::services;
use crate::error::ApiError;

pub fn create_player(data:requests::CreatePlayerRequest,repo :&Repo) -> Result<(),Box<dyn Error>>
{
	let email_re = regex::Regex::new(r#"(?:[a-z0-9!#$%&'*+\/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+\/=?^_`{|}~-]+)*|\"(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21\\x23-\\x5b\\x5d-\\x7f]|\\\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])*\")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21-\\x5a\\x53-\\x7f]|\\\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])+)\\])"#).unwrap();
	let salt = services::crypto::generate_salt();
	let pwd = services::crypto::hash_password(&data.password,&salt);

	if !email_re.is_match(&data.email)
	{
		return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"Email is bad")));
	}

    let res = repo.player_repo.insert_player(&Player{
		id:None,
		username: data.username.clone(),
		email: data.email.clone(),
		password_salt: salt,
		password_hash: pwd,
		token:None
	})?;

	Ok(())
}

pub fn login_player(login_info: requests::LoginPlayerRequest, repo:&Repo) -> Result<responses::LoginPlayerResponse,ApiError>
{
}
