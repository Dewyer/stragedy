use crate::repos::Repo;
use crate::models::player::{Player, PlayerToken};
use crate::models;
use lib::requests;
use lib::responses;
use std::error::Error;
use crate::services;
use crate::error::ApiError;
use chrono::prelude::*;
use std::ops::Add;
use chrono::Duration;
use bson::Document;
use crate::repos::generic_repo::GenericRepo;
use crate::helpers::updater::UpdateExp;
use crate::helpers::query::QueryExp;

pub fn create_player(data:requests::CreatePlayerRequest,repo :&Repo) -> Result<(),ApiError>
{
	let email_re = regex::Regex::new(r#"[^@]+@[^\.]+\..+"#).unwrap();
	let salt = services::crypto::generate_salt();
	let pwd = services::crypto::hash_password(&data.password,&salt);

	if !email_re.is_match(&data.email)
	{
		return Err(ApiError::new("email-bad"));
	}

	let user_already = repo.player_repo.find_by_filter(doc!{"$or":[{"username": &data.username},{"email":&data.email}] });
	if user_already.is_some()
	{
		return Err(ApiError::new("user-exists"));
	}

    let res = repo.player_repo.insert_model(&Player{
		id:Some(bson::oid::ObjectId::new().unwrap()),
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
	let player = repo.player_repo.find_by_filter(doc!{"username": login_info.username});
	if player.is_none()
	{
		return Err(ApiError::new("no-user"));
	}
	let player = player.unwrap();

	if services::crypto::has_password(&login_info.password,&player)
	{
		let player_id = player.id.as_ref().unwrap().to_hex();
		let claim = models::player::JwtClaims
		{
			player_id:player_id.clone(),
			token:services::crypto::generate_random_crypto_string(64)
		};

		let jwt_token = services::crypto::issue_jwt(&claim)?;
		let expiration = Utc::now() + Duration::days(1);
		let player_token = PlayerToken
		{
			sec_token:claim.token,
			expiration
		};
		let player_token_bson = bson::to_bson(&player_token).unwrap();
		let mut to_update = UpdateExp::new().set("token",player_token).doc();
		let mut filter = QueryExp::new_by_id(player.id.as_ref().unwrap()).doc();

		println!("was to update: {:?}",to_update);

		repo.player_repo.update_by_doc(&filter,&to_update)?;

		Ok(responses::LoginPlayerResponse{
			jwt:jwt_token,
			expiration_tick:expiration.timestamp_millis()
		})
	}
	else
	{
		Err(ApiError::new("wrong-password"))
	}
}
