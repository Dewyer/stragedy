use bson;
use chrono::prelude::*;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::State;
use crate::repos::Repo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player
{
	#[serde(rename = "_id")]
	pub id: Option<bson::oid::ObjectId>,
	pub username:String,
	pub email:String,
	pub password_salt:String,
	pub password_hash:String,

	pub token:Option<PlayerToken>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerToken
{
	pub sec_token:String,
	pub expiration:DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
	pub player_id: String,
	pub token: String,
	pub exp: u64
}

#[derive(Debug)]
pub enum AuthFail
{
	NoToken,
	InvalidToken,
	Expired,
	DbError
}

impl<'a, 'r> FromRequest<'a, 'r> for Player {
    type Error = AuthFail;

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error>
	{
		let repo = request.guard::<State<Repo>>();

		if let Outcome::Success(rr) = repo
		{
			let repo = rr.inner();
			let auth_header = request.headers().get_one("Authorization");
			if auth_header.is_none()
			{
				Outcome::Failure((Status::BadRequest,AuthFail::NoToken))
			}
			else
			{
				let auth_header = auth_header.unwrap();
				let jwt_decoded = jsonwebtoken::decode::<JwtClaims>(auth_header, &jsonwebtoken::DecodingKey::from_base64_secret(dotenv!("JWT_SECRET")).unwrap(), &jsonwebtoken::Validation::default());
				match jwt_decoded
				{
					Ok(jwt)=>
					{
						let oid = bson::oid::ObjectId::with_string(&jwt.claims.player_id);
						if let Err(err) = oid
						{
							return Outcome::Failure((Status::BadRequest,AuthFail::InvalidToken));
						}
						let target_user = repo.player_repo.find_by_id(oid.unwrap());
						match target_user
						{
							Some(user)=> {
								match user.token.as_ref()
								{
									Some(tok)=>
									{
										if &tok.sec_token == &jwt.claims.token
										{
											Outcome::Success(user)
										}
										else
										{
											Outcome::Failure((Status::BadRequest,AuthFail::InvalidToken))
										}
									},
									None=>Outcome::Failure((Status::BadRequest,AuthFail::InvalidToken))
								}
							},
							None=> Outcome::Failure((Status::BadRequest,AuthFail::InvalidToken))
						}
					}
					Err(eke)=>{
						println!("why fail jwt : {:?}",eke);
						Outcome::Failure((Status::BadRequest,AuthFail::InvalidToken))
					}
				}
			}
		}
		else
		{
			Outcome::Failure((Status::BadRequest,AuthFail::DbError))
		}
    }
}
