pub mod player_repo;

use mongodb::sync::{Client,Database};
use rocket::request::{FromRequest,Request,Outcome};
use rocket::http::Status;
use std::error::Error;
use crate::repos::player_repo::PlayerRepo;

pub struct Repo
{
    client:Client,
	database:Database,

	pub player_repo:PlayerRepo
}

impl Repo
{
    pub fn new() -> Result<Self,Box<dyn Error>>
    {
        let constr = dotenv!("MONGO_CON");
        let client = Client::with_uri_str(constr)?;
        let db = client.database("stragedy");
		let pc = db.collection("players");

        Ok(Repo
        {
            client:client,
			database:db,
			player_repo: PlayerRepo::new(pc)
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Repo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Repo, ()> {
        let repo = Repo::new();
        match repo {
            Ok(db) => Outcome::Success(db),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
