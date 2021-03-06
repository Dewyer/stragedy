use crate::repos::Repo;
use lib::models::player::{Player, PlayerToken,JwtClaims};
use lib::requests;
use lib::responses;
use std::error::Error;
use crate::services;
use chrono::prelude::*;
use std::ops::Add;
use chrono::Duration;
use bson::Document;
use crate::repos::generic_repo::GenericRepo;
use crate::helpers::updater::UpdateExp;
use crate::helpers::query::QueryExp;
use lib::error::AuthError;
use lib::models::planet::{Planet, PlanetCoordinate};

pub fn create_player(data:requests::CreatePlayerRequest,repo :&Repo) -> Result<(),AuthError>
{
	let email_re = lib::regex::get_email_regex();
	let salt = services::crypto::generate_salt();
	let pwd = services::crypto::hash_password(&data.password,&salt);

	if !email_re.is_match(&data.email)
	{
		return Err(AuthError::BadEmail);
	}

	let user_already = repo.player_repo.find_by_filter(doc!{"$or":[{"username": &data.username},{"email":&data.email}] });
	if user_already.is_some()
	{
		return Err(AuthError::UserExists);
	}

	let mut player = Player::new(&data.username,&data.email,&pwd,&salt);
	let starter_coordinate = services::planet_manager::get_starter_planet_coordinate(&repo)?;
	let starter_planet = Planet::new(player.get_cpy_id(),starter_coordinate);

	player.controlled_planet_ids.push(starter_planet.get_cpy_id());

	let _res2 = repo.planet_repo.insert_model(&starter_planet)?;
    let _res = repo.player_repo.insert_model(&player)?;

	Ok(())
}

pub fn login_player(login_info: requests::LoginPlayerRequest, repo:&Repo) -> Result<responses::LoginPlayerResponse,AuthError>
{
	let player = repo.player_repo.find_by_filter(doc!{"username": login_info.username});
	if player.is_none()
	{
		return Err(AuthError::NoUser);
	}
	let player = player.unwrap();

	if services::crypto::has_password(&login_info.password,&player)
	{
		let player_id = player.id.as_ref().unwrap().to_hex();
		let expiration = Utc::now() + Duration::days(1);
		let claim = JwtClaims
		{
			player_id:player_id.clone(),
			token:services::crypto::generate_random_crypto_string(64),
			exp:expiration.clone().timestamp_millis() as u64
		};

		let jwt_token = services::crypto::issue_jwt(&claim)?;
		let player_token = PlayerToken
		{
			sec_token:claim.token,
			expiration
		};
		let player_token_bson = bson::to_bson(&player_token).unwrap();
		let to_update = UpdateExp::new().set("token",player_token).doc();
		let filter = QueryExp::new_by_id(player.id.as_ref().unwrap()).doc();

		println!("was to update: {:?}",to_update);

		repo.player_repo.update_by_doc(&filter,&to_update)?;

		Ok(responses::LoginPlayerResponse{
			jwt:jwt_token,
			expiration_tick:expiration.timestamp_millis()
		})
	}
	else
	{
		Err(AuthError::WrongPassword)
	}
}
