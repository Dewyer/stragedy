use regex::Regex;

pub fn get_email_regex() -> Regex
{
	regex::Regex::new(r#"[^@]+@[^\.]+\..+"#).unwrap()
}

pub fn get_password_regex() -> Regex
{
	Regex::new(r#"^(.{6,})"#).unwrap()
}
