use rsgen::{gen_random_string, OutputCharsType};
use sha2::{Digest,Sha256};

pub fn generate_salt() ->  String
{
	let output_chars_type = OutputCharsType::LatinAlphabetAndNumeric {
		use_upper_case: true,
		use_lower_case: true,
	};
	gen_random_string(32, output_chars_type)
}

pub fn hash_password(pwd:&str,salt:&str) -> String
{
	let mut hasher = Sha256::new();
	hasher.input(format!("{}{}{}",salt,pwd,salt).as_bytes());
	let res = hasher.result();

	hex::encode(res)
}
