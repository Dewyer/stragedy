use rsgen::{gen_random_string, OutputCharsType};
use sha2::{Digest,Sha256};
use lib::models::player::{Player, JwtClaims};
use std::ptr::hash;
use std::error::Error;
use lib::error::ApiError;

pub fn generate_random_crypto_string(len:usize) -> String
{
	let output_chars_type = OutputCharsType::LatinAlphabetAndNumeric {
		use_upper_case: true,
		use_lower_case: true,
	};
	gen_random_string(len, output_chars_type)
}

pub fn generate_salt() ->  String
{
	generate_random_crypto_string(32)
}

pub fn hash_password(pwd:&str,salt:&str) -> String
{
	let mut hasher = Sha256::new();
	hasher.input(format!("{}{}{}",salt,pwd,salt).as_bytes());
	let res = hasher.result();

	hex::encode(res)
}

pub fn has_password(pwd:&str,player:&Player ) -> bool
{
	let hashed_test = hash_password(pwd,&player.password_salt);
	hashed_test == player.password_hash
}

pub fn issue_jwt(claim:&JwtClaims) -> Result<String,ApiError>
{
	match jsonwebtoken::encode(
		&jsonwebtoken::Header::default(),
		claim,
		&jsonwebtoken::EncodingKey::from_base64_secret(dotenv!("JWT_SECRET")).unwrap())
	{
		Ok(val) => Ok(val),
		Err(er) => Err(ApiError::Other("jwt-err".to_string()))
	}
}
