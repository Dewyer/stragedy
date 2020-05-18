use crate::repos::Repo;
use crate::models::player::Player;
use crate::services;
use lib::requests;

pub fn seed(repo:&Repo) -> bool
{
	if !seed_players(repo)
	{
		return false;
	}
	true
}

fn seed_players(repo:&Repo) -> bool
{
	let user_already = repo.player_repo.find_by_filter(doc!{"username":"test1"});
	if user_already.is_none()
	{
		services::auth::create_player(requests::CreatePlayerRequest{
			username: "test1".to_string(),
			email: "test@gmail.com".to_string(),
			password: "Test12345".to_string()
		},repo).is_ok()
	}
	else{ true }
}
